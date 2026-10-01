//! A minimal HTTP/1.1 client. The goldens pin how the server treats the raw
//! request target and the `Host` header, so the client sends both untouched,
//! which rules out a client library that normalizes either.

use std::collections::BTreeMap;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

/// The `Host` every golden request carries. The server echoes it in
/// `links.self`, so a fixed value keeps the goldens independent of the port.
pub const HOST: &str = "golden.pgconfig.test";

#[derive(Debug, PartialEq)]
pub struct Response {
    pub status: u16,
    /// Header names are lowercased. A repeated header keeps its last value.
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

/// A keep-alive connection to one server.
pub struct Client {
    addr: SocketAddr,
    stream: Option<BufReader<TcpStream>>,
}

impl Client {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr, stream: None }
    }

    /// Sends one request and reads its response. A connection the server
    /// closed between two requests is reopened once.
    pub fn request(
        &mut self,
        method: &str,
        target: &str,
        headers: &BTreeMap<String, String>,
    ) -> io::Result<Response> {
        let reused = self.stream.is_some();
        match self.exchange(method, target, headers) {
            Err(_) if reused => {
                self.stream = None;
                self.exchange(method, target, headers)
            }
            result => result,
        }
    }

    fn exchange(
        &mut self,
        method: &str,
        target: &str,
        headers: &BTreeMap<String, String>,
    ) -> io::Result<Response> {
        let mut stream = match self.stream.take() {
            Some(stream) => stream,
            None => {
                let stream = TcpStream::connect_timeout(&self.addr, Duration::from_secs(5))?;
                stream.set_read_timeout(Some(Duration::from_secs(30)))?;
                stream.set_nodelay(true)?;
                BufReader::new(stream)
            }
        };

        let mut request = format!("{method} {target} HTTP/1.1\r\nHost: {HOST}\r\n");
        for (name, value) in headers {
            request.push_str(&format!("{name}: {value}\r\n"));
        }
        if method != "GET" && method != "HEAD" {
            request.push_str("Content-Length: 0\r\n");
        }
        request.push_str("\r\n");
        stream.get_mut().write_all(request.as_bytes())?;

        let (response, keep_alive) = read_response(&mut stream, method == "HEAD")?;
        if keep_alive {
            self.stream = Some(stream);
        }
        Ok(response)
    }
}

/// Reads one response. The flag tells whether the connection can carry
/// another request.
fn read_response(reader: &mut impl BufRead, head: bool) -> io::Result<(Response, bool)> {
    let status_line = read_line(reader)?;
    let status = status_line
        .split(' ')
        .nth(1)
        .and_then(|code| code.parse::<u16>().ok())
        .ok_or_else(|| io::Error::other(format!("malformed status line: {status_line:?}")))?;

    let mut headers = BTreeMap::new();
    loop {
        let line = read_line(reader)?;
        if line.is_empty() {
            break;
        }
        let (name, value) = line
            .split_once(':')
            .ok_or_else(|| io::Error::other(format!("malformed header: {line:?}")))?;
        headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
    }

    let close = headers
        .get("connection")
        .is_some_and(|v| v.eq_ignore_ascii_case("close"));
    let chunked = headers
        .get("transfer-encoding")
        .is_some_and(|v| v.eq_ignore_ascii_case("chunked"));
    let length = headers
        .get("content-length")
        .and_then(|v| v.parse::<usize>().ok());

    let mut body = Vec::new();
    let mut keep_alive = !close;
    if head || status == 204 || status == 304 {
        // No body, whatever the headers say.
    } else if chunked {
        loop {
            let size_line = read_line(reader)?;
            let size = usize::from_str_radix(size_line.split(';').next().unwrap_or("").trim(), 16)
                .map_err(|_| io::Error::other(format!("malformed chunk size: {size_line:?}")))?;
            if size == 0 {
                // Trailers, then the final blank line.
                while !read_line(reader)?.is_empty() {}
                break;
            }
            let start = body.len();
            body.resize(start + size, 0);
            reader.read_exact(&mut body[start..])?;
            read_line(reader)?;
        }
    } else if let Some(length) = length {
        body.resize(length, 0);
        reader.read_exact(&mut body)?;
    } else {
        reader.read_to_end(&mut body)?;
        keep_alive = false;
    }

    Ok((
        Response {
            status,
            headers,
            body,
        },
        keep_alive,
    ))
}

fn read_line(reader: &mut impl BufRead) -> io::Result<String> {
    let mut line = String::new();
    if reader.read_line(&mut line)? == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "connection closed",
        ));
    }
    Ok(line.trim_end_matches(['\r', '\n']).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(raw: &str, head: bool) -> (Response, bool) {
        read_response(&mut raw.as_bytes(), head).unwrap()
    }

    #[test]
    fn reads_a_body_by_content_length_and_leaves_the_rest() {
        let raw =
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nhelloHTTP/1.1";
        let mut reader = raw.as_bytes();

        let (response, keep_alive) = read_response(&mut reader, false).unwrap();

        assert_eq!(response.status, 200);
        assert_eq!(response.headers["content-type"], "text/plain");
        assert_eq!(response.body, b"hello");
        assert!(keep_alive);
        assert_eq!(reader, b"HTTP/1.1");
    }

    #[test]
    fn decodes_a_chunked_body() {
        let raw = "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n4\r\nWiki\r\n5\r\npedia\r\n0\r\n\r\n";

        let (response, keep_alive) = parse(raw, false);

        assert_eq!(response.body, b"Wikipedia");
        assert!(keep_alive);
    }

    #[test]
    fn reads_to_the_end_without_a_length() {
        let (response, keep_alive) = parse("HTTP/1.1 404 Not Found\r\n\r\ngone", false);

        assert_eq!(response.status, 404);
        assert_eq!(response.body, b"gone");
        assert!(!keep_alive);
    }

    #[test]
    fn a_head_response_has_no_body() {
        let (response, _) = parse("HTTP/1.1 200 OK\r\nContent-Length: 253\r\n\r\n", true);

        assert!(response.body.is_empty());
    }

    #[test]
    fn connection_close_ends_the_keep_alive() {
        let raw = "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";

        let (_, keep_alive) = parse(raw, false);

        assert!(!keep_alive);
    }
}
