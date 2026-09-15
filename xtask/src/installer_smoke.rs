use anyhow::{Context, Result, bail};
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tempfile::Builder;

struct Execution {
  code: Option<i32>,
  stdout: String,
  stderr: String,
}

struct ArtifactServer {
  address: SocketAddr,
  corrupt_downloads: Arc<AtomicBool>,
  shutdown: Arc<AtomicBool>,
  thread: Option<JoinHandle<Result<()>>>,
}

impl ArtifactServer {
  fn start(artifacts: PathBuf) -> Result<Self> {
    let listener =
      TcpListener::bind(("127.0.0.1", 0)).context("could not start the installer smoke server")?;
    let address = listener.local_addr()?;
    let corrupt_downloads = Arc::new(AtomicBool::new(false));
    let shutdown = Arc::new(AtomicBool::new(false));
    let server_corruption = Arc::clone(&corrupt_downloads);
    let server_shutdown = Arc::clone(&shutdown);
    let thread = thread::spawn(move || {
      serve_artifacts(listener, &artifacts, &server_corruption, &server_shutdown)
    });
    Ok(Self {
      address,
      corrupt_downloads,
      shutdown,
      thread: Some(thread),
    })
  }

  fn download_url(&self) -> String {
    format!("http://{}", self.address)
  }

  fn set_corrupt_downloads(&self, corrupt: bool) {
    self.corrupt_downloads.store(corrupt, Ordering::SeqCst);
  }

  fn finish(&mut self) -> Result<()> {
    self.shutdown.store(true, Ordering::SeqCst);
    let _ = TcpStream::connect(self.address);
    let Some(thread) = self.thread.take() else {
      return Ok(());
    };
    thread
      .join()
      .map_err(|_| anyhow::anyhow!("installer smoke server thread panicked"))?
  }
}

impl Drop for ArtifactServer {
  fn drop(&mut self) {
    let _ = self.finish();
  }
}

pub fn smoke(artifacts: &Path) -> Result<()> {
  let artifacts = absolute(artifacts)?;
  if !artifacts.is_dir() {
    bail!(
      "native release artifact directory does not exist: {}",
      artifacts.display()
    );
  }
  let temporary = Builder::new()
    .prefix("arcantry-installer-smoke-")
    .tempdir()?;
  let mut server = ArtifactServer::start(artifacts.clone())?;
  let download_url = server.download_url();

  let result = (|| {
    for (index, (command, arguments)) in installer_commands(&artifacts).into_iter().enumerate() {
      let install_directory = temporary.path().join(format!("install-{index}"));
      let installed = execute(&command, &arguments, &install_directory, &download_url)?;
      if !installed.succeeded() {
        bail!(
          "{} installer failed: {}",
          command.to_string_lossy(),
          installed.failure_message()
        );
      }

      let executable = install_directory.join("bin").join(if cfg!(windows) {
        "arcantry.exe"
      } else {
        "arcantry"
      });
      let version = execute(
        executable.as_os_str(),
        &[OsString::from("--version")],
        &install_directory,
        &download_url,
      )?;
      if !version.succeeded() || version.stdout.trim() != env!("CARGO_PKG_VERSION") {
        bail!(
          "{} installer produced an invalid native executable: {}",
          command.to_string_lossy(),
          version.failure_message()
        );
      }

      server.set_corrupt_downloads(true);
      let rejected = execute(
        &command,
        &arguments,
        &temporary.path().join(format!("corrupt-{index}")),
        &download_url,
      )?;
      server.set_corrupt_downloads(false);
      let rejection_output = format!("{}\n{}", rejected.stdout, rejected.stderr).to_lowercase();
      if rejected.succeeded() || !rejection_output.contains("checksum mismatch") {
        bail!(
          "{} installer did not reject a corrupted archive",
          command.to_string_lossy()
        );
      }
    }
    Ok(())
  })();
  let server_result = server.finish();
  result?;
  server_result?;

  println!(
    "{} passed install, version, and checksum rejection smoke tests.",
    if cfg!(windows) {
      "PowerShell installer"
    } else {
      "Shell and PowerShell installers"
    }
  );
  Ok(())
}

impl Execution {
  fn succeeded(&self) -> bool {
    self.code == Some(0)
  }

  fn failure_message(&self) -> &str {
    if !self.stderr.trim().is_empty() {
      self.stderr.trim()
    } else if !self.stdout.trim().is_empty() {
      self.stdout.trim()
    } else {
      "command exited without output"
    }
  }
}

fn installer_commands(artifacts: &Path) -> Vec<(OsString, Vec<OsString>)> {
  let powershell = (
    OsString::from("pwsh"),
    vec![
      OsString::from("-NoProfile"),
      OsString::from("-File"),
      artifacts.join("arcantry-installer.ps1").into_os_string(),
      OsString::from("-NoModifyPath"),
    ],
  );
  if cfg!(windows) {
    vec![powershell]
  } else {
    vec![
      (
        OsString::from("bash"),
        vec![
          artifacts.join("arcantry-installer.sh").into_os_string(),
          OsString::from("--no-modify-path"),
        ],
      ),
      powershell,
    ]
  }
}

fn execute(
  command: &OsStr,
  arguments: &[OsString],
  install_directory: &Path,
  download_url: &str,
) -> Result<Execution> {
  let output = Command::new(command)
    .args(arguments)
    .env("CARGO_DIST_FORCE_INSTALL_DIR", install_directory)
    .env("INSTALLER_DOWNLOAD_URL", download_url)
    .stdin(Stdio::null())
    .output()
    .with_context(|| format!("failed to execute {}", command.to_string_lossy()))?;
  Ok(Execution {
    code: output.status.code(),
    stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
    stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
  })
}

fn serve_artifacts(
  listener: TcpListener,
  artifacts: &Path,
  corrupt_downloads: &AtomicBool,
  shutdown: &AtomicBool,
) -> Result<()> {
  while !shutdown.load(Ordering::SeqCst) {
    let (mut stream, _) = listener
      .accept()
      .context("installer smoke server could not accept a connection")?;
    if shutdown.load(Ordering::SeqCst) {
      break;
    }
    respond(
      &mut stream,
      artifacts,
      corrupt_downloads.load(Ordering::SeqCst),
    )?;
  }
  Ok(())
}

fn respond(stream: &mut TcpStream, artifacts: &Path, corrupt: bool) -> Result<()> {
  stream.set_read_timeout(Some(Duration::from_secs(5)))?;
  let mut request = Vec::new();
  let mut buffer = [0_u8; 1024];
  while request.len() < 8 * 1024 && !request.ends_with(b"\r\n\r\n") {
    let read = stream.read(&mut buffer)?;
    if read == 0 {
      break;
    }
    request.extend_from_slice(&buffer[..read]);
  }
  let request_text = String::from_utf8_lossy(&request);
  let target = request_text
    .lines()
    .next()
    .and_then(|line| line.split_whitespace().nth(1));
  let Some(mut bytes) = target.and_then(|value| load_artifact(artifacts, value)) else {
    stream
      .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")?;
    return Ok(());
  };
  if corrupt && target.is_some_and(is_archive_request) {
    bytes.extend_from_slice(b"corrupt");
  }
  write!(
    stream,
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
    bytes.len()
  )?;
  stream.write_all(&bytes)?;
  Ok(())
}

fn load_artifact(artifacts: &Path, request_target: &str) -> Option<Vec<u8>> {
  let path = request_target.split('?').next()?;
  let name = path.rsplit('/').find(|segment| !segment.is_empty())?;
  if name == "." || name == ".." || name.contains('\\') {
    return None;
  }
  fs::read(artifacts.join(name)).ok()
}

fn is_archive_request(request_target: &str) -> bool {
  let path = request_target.split('?').next().unwrap_or(request_target);
  path.ends_with(".zip") || path.ends_with(".tar.xz")
}

fn absolute(path: &Path) -> Result<PathBuf> {
  if path.is_absolute() {
    Ok(path.to_owned())
  } else {
    Ok(std::env::current_dir()?.join(path))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn loads_only_the_requested_artifact_basename() {
    let temporary = tempfile::tempdir().unwrap();
    fs::write(temporary.path().join("archive.zip"), b"archive").unwrap();
    assert_eq!(
      load_artifact(temporary.path(), "/downloads/archive.zip?source=smoke").unwrap(),
      b"archive"
    );
    assert!(load_artifact(temporary.path(), "/downloads/missing.zip").is_none());
    assert!(load_artifact(temporary.path(), "/downloads/..\\secret").is_none());
  }

  #[test]
  fn corrupts_only_release_archives() {
    assert!(is_archive_request("/arcantry.zip"));
    assert!(is_archive_request("/arcantry.tar.xz?download=1"));
    assert!(!is_archive_request("/arcantry-installer.ps1"));
  }

  #[test]
  fn selects_the_native_installers_for_the_host() {
    let commands = installer_commands(Path::new("artifacts"));
    if cfg!(windows) {
      assert_eq!(commands.len(), 1);
      assert_eq!(commands[0].0, "pwsh");
    } else {
      assert_eq!(commands.len(), 2);
      assert_eq!(commands[0].0, "bash");
      assert_eq!(commands[1].0, "pwsh");
    }
  }
}
