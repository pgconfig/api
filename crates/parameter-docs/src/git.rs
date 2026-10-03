//! Reads a PostgreSQL git checkout at a release tag, without touching its
//! working tree.

use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct Checkout {
    dir: PathBuf,
}

impl Checkout {
    pub fn open(dir: &Path) -> Result<Self, String> {
        let checkout = Checkout {
            dir: dir.to_path_buf(),
        };
        let tags = checkout.git(&["tag", "--list", "REL_1*_0"])?;
        if tags.trim().is_empty() {
            return Err(format!(
                "{} has no PostgreSQL release tags. Clone https://github.com/postgres/postgres.git there, or fetch its tags",
                dir.display()
            ));
        }
        Ok(checkout)
    }

    /// The newest release tag of a major version: `REL_18_6` for `18`,
    /// `REL9_6_24` for `9.6`. Betas and release candidates do not count.
    pub fn release_tag(&self, major: &str) -> Result<String, String> {
        let prefix = match major.split_once('.') {
            Some((nine, minor)) => format!("REL{nine}_{minor}_"),
            None => format!("REL_{major}_"),
        };
        let tags = self.git(&["tag", "--list", &format!("{prefix}*")])?;
        tags.lines()
            .filter_map(|tag| {
                let patch: u32 = tag.strip_prefix(&prefix)?.parse().ok()?;
                Some((patch, tag))
            })
            .max()
            .map(|(_, tag)| tag.to_string())
            .ok_or_else(|| {
                format!(
                    "no release tag of PostgreSQL {major} in {}",
                    self.dir.display()
                )
            })
    }

    /// The paths of the files under `dir` at `tag`.
    pub fn files(&self, tag: &str, dir: &str) -> Result<Vec<String>, String> {
        let listing = self.git(&["ls-tree", "-r", "--name-only", tag, "--", dir])?;
        Ok(listing.lines().map(str::to_string).collect())
    }

    /// The lines that match the extended regular expression `pattern` in the
    /// files under `paths` at `tag`, each with the path it is in.
    pub fn grep(
        &self,
        tag: &str,
        pattern: &str,
        paths: &[&str],
    ) -> Result<Vec<(String, String)>, String> {
        let mut args = vec!["grep", "-E", "-e", pattern, tag, "--"];
        args.extend_from_slice(paths);
        let output = self
            .command(&args)
            .output()
            .map_err(|err| err.to_string())?;
        // git grep exits with 1 when nothing matches.
        if !output.status.success() && output.status.code() != Some(1) {
            return Err(String::from_utf8_lossy(&output.stderr).into_owned());
        }
        let prefix = format!("{tag}:");
        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter_map(|line| {
                let (path, text) = line.strip_prefix(&prefix)?.split_once(':')?;
                Some((path.to_string(), text.to_string()))
            })
            .collect())
    }

    /// Reads files one after the other through a single git process.
    pub fn reader(&self) -> Result<Reader, String> {
        let mut child = self
            .command(&["cat-file", "--batch"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|err| format!("cannot run git: {err}"))?;
        let stdin = child.stdin.take().expect("a piped stdin");
        let stdout = BufReader::new(child.stdout.take().expect("a piped stdout"));
        Ok(Reader {
            child,
            stdin,
            stdout,
        })
    }

    fn command(&self, args: &[&str]) -> Command {
        let mut command = Command::new("git");
        command.arg("-C").arg(&self.dir).args(args);
        command
    }

    fn git(&self, args: &[&str]) -> Result<String, String> {
        let output = self
            .command(args)
            .output()
            .map_err(|err| format!("cannot run git: {err}"))?;
        if !output.status.success() {
            return Err(format!(
                "git {}: {}",
                args.join(" "),
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

pub struct Reader {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Reader {
    /// The content of `path` at `tag`, or `None` when the file does not exist
    /// there.
    pub fn read(&mut self, tag: &str, path: &str) -> Result<Option<String>, String> {
        let io = |err: std::io::Error| format!("reading {tag}:{path}: {err}");
        writeln!(self.stdin, "{tag}:{path}").map_err(io)?;
        self.stdin.flush().map_err(io)?;
        let mut header = String::new();
        self.stdout.read_line(&mut header).map_err(io)?;
        if header.trim_end().ends_with(" missing") {
            return Ok(None);
        }
        let size: usize = header
            .split_whitespace()
            .nth(2)
            .and_then(|size| size.parse().ok())
            .ok_or_else(|| format!("reading {tag}:{path}: git answered {header:?}"))?;
        // The content is followed by a newline.
        let mut content = vec![0; size + 1];
        self.stdout.read_exact(&mut content).map_err(io)?;
        content.pop();
        String::from_utf8(content)
            .map(Some)
            .map_err(|_| format!("{tag}:{path} is not UTF-8"))
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
