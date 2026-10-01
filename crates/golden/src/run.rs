//! Starts the binaries under test.

use std::collections::BTreeMap;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::http::Client;

/// A server process that is killed when the value is dropped.
pub struct Server {
    child: Child,
    addr: SocketAddr,
}

impl Server {
    /// Starts `bin --port <free port>` in `cwd` and waits until it answers
    /// `/v1/version`. The Go API reads `rules.yml` and `pg-docs.yml` from its
    /// working directory, so `cwd` is the repository root.
    pub fn spawn(bin: &Path, cwd: &Path) -> io::Result<Self> {
        let port = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?
            .local_addr()?
            .port();
        let child = Command::new(bin)
            .args(["--port", &port.to_string()])
            .current_dir(cwd)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|err| io::Error::other(format!("cannot start {}: {err}", bin.display())))?;
        let mut server = Self {
            child,
            addr: SocketAddr::from((Ipv4Addr::LOCALHOST, port)),
        };
        server.wait_until_ready()?;
        Ok(server)
    }

    pub fn client(&self) -> Client {
        Client::new(self.addr)
    }

    fn wait_until_ready(&mut self) -> io::Result<()> {
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            if let Some(status) = self.child.try_wait()? {
                return Err(io::Error::other(format!(
                    "the server exited early: {status}"
                )));
            }
            let ready = self
                .client()
                .request("GET", "/v1/version", &BTreeMap::new())
                .is_ok_and(|response| response.status == 200);
            if ready {
                return Ok(());
            }
            if Instant::now() > deadline {
                return Err(io::Error::other(
                    "the server did not become ready in 20 seconds",
                ));
            }
            thread::sleep(Duration::from_millis(50));
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

pub struct CliOutput {
    pub exit_code: i32,
    pub stdout: String,
}

/// Runs the CLI once. `home` is an empty directory: the Go CLI looks for
/// `$HOME/.pgconfigctl.yaml` and announces it on stdout when it finds one.
pub fn cli(bin: &Path, args: &[String], home: &Path) -> io::Result<CliOutput> {
    let output = Command::new(bin)
        .args(args)
        .env("HOME", home)
        .stdin(Stdio::null())
        .output()
        .map_err(|err| io::Error::other(format!("cannot run {}: {err}", bin.display())))?;
    let exit_code = output
        .status
        .code()
        .ok_or_else(|| io::Error::other(format!("the CLI was killed by a signal: {args:?}")))?;
    let stdout = String::from_utf8(output.stdout).map_err(io::Error::other)?;
    Ok(CliOutput { exit_code, stdout })
}
