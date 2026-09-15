use crate::native_targets::{self, TARGETS};
use anyhow::{Context, Result, bail};
use serde_json::{Map, Value, json};
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tempfile::Builder;

const REGISTRY_USER: &str = "arcantry-smoke";
const REGISTRY_PASSWORD: &str = "arcantry-local-registry-smoke";

pub fn smoke(root: &Path, archives: &Path) -> Result<()> {
  let root = absolute(root)?;
  let archives = absolute(archives)?;
  let temporary = Builder::new()
    .prefix("arcantry-registry-smoke-")
    .tempdir()?;
  let port = available_port()?;
  let registry_url = format!("http://127.0.0.1:{port}");
  let config = temporary.path().join("verdaccio.yaml");
  fs::write(&config, registry_config(temporary.path())?)?;
  let log_path = temporary.path().join("verdaccio.log");
  let mut registry = start_registry(&root, &config, &log_path, port)?;

  let result = run_smoke(
    &root,
    &archives,
    temporary.path(),
    port,
    &registry_url,
    &log_path,
  );
  let stop_result = stop_registry(&mut registry);
  result?;
  stop_result?;
  println!("Local registry smoke passed for all six npm platform packages.");
  Ok(())
}

fn run_smoke(
  root: &Path,
  archives_root: &Path,
  temporary: &Path,
  port: u16,
  registry_url: &str,
  log_path: &Path,
) -> Result<()> {
  wait_for_registry(port, log_path)?;
  let token = create_registry_user(port)?;
  let user_config = temporary.join("npmrc");
  fs::write(
    &user_config,
    format!("registry={registry_url}\n//127.0.0.1:{port}/:_authToken={token}\n"),
  )?;

  let archives = npm_archives(archives_root)?;
  for (index, archive) in archives.iter().enumerate() {
    let extraction = temporary.join("publish").join(index.to_string());
    fs::create_dir_all(&extraction)?;
    run(
      "tar",
      [
        OsStr::new("-xzf"),
        archive.as_os_str(),
        OsStr::new("-C"),
        extraction.as_os_str(),
      ],
      temporary,
      Some(&user_config),
    )?;
    run(
      "nub",
      [
        OsStr::new("publish"),
        OsStr::new("--registry"),
        OsStr::new(registry_url),
        OsStr::new("--ignore-scripts"),
        OsStr::new("--no-git-checks"),
        OsStr::new("--access"),
        OsStr::new("public"),
      ],
      &extraction.join("package"),
      Some(&user_config),
    )?;
  }

  let version = package_version(root)?;
  let host = native_targets::host()?;
  for target in &TARGETS {
    let install = temporary
      .join("install")
      .join(format!("{}-{}", target.os, target.cpu));
    fs::create_dir_all(&install)?;
    fs::write(
      install.join("package.json"),
      install_manifest(target.os, target.cpu, target.package_name, &version)?,
    )?;
    run(
      "nub",
      [
        OsStr::new("install"),
        OsStr::new("--registry"),
        OsStr::new(registry_url),
        OsStr::new("--ignore-scripts"),
        OsStr::new("--minimum-release-age"),
        OsStr::new("0"),
        OsStr::new("--os"),
        OsStr::new(target.os),
        OsStr::new("--cpu"),
        OsStr::new(target.cpu),
      ],
      &install,
      Some(&user_config),
    )?;
    fs::read_to_string(
      install
        .join("node_modules")
        .join(
          target
            .package_name
            .replace('/', std::path::MAIN_SEPARATOR_STR),
        )
        .join("package.json"),
    )
    .with_context(|| format!("registry install omitted {}", target.package_name))?;
    run(
      "nub",
      [
        OsStr::new("--node"),
        OsStr::new("-e"),
        OsStr::new(
          "await Promise.all(['arcantry','arcantry/catalog','arcantry/repository','arcantry/project','arcantry/release'].map(async (specifier) => { let exposed = false; try { await import(specifier); exposed = true; } catch {} if (exposed) throw new Error(`${specifier} unexpectedly exposes a JavaScript API`); }))",
        ),
      ],
      &install,
      Some(&user_config),
    )?;
    if target == host {
      for (runner, arguments) in registry_runners() {
        let output = run(runner, arguments, &install, Some(&user_config))?;
        let actual = String::from_utf8(output)?.trim().to_owned();
        if actual != version {
          bail!("{runner} reported {actual}, expected {version} from the registry-installed CLI.");
        }
      }
    }
  }
  Ok(())
}

fn available_port() -> Result<u16> {
  let listener = TcpListener::bind(("127.0.0.1", 0))?;
  Ok(listener.local_addr()?.port())
}

fn registry_config(temporary: &Path) -> Result<String> {
  let storage = serde_json::to_string(&temporary.join("storage").to_string_lossy())?;
  let htpasswd = serde_json::to_string(&temporary.join("htpasswd").to_string_lossy())?;
  Ok(format!(
    "storage: {storage}\nauth:\n  htpasswd:\n    file: {htpasswd}\n    max_users: 1000\nuplinks:\n  npmjs:\n    url: https://registry.npmjs.org/\npackages:\n  '@*/*':\n    access: $all\n    publish: $all\n    unpublish: $all\n    proxy: npmjs\n  '**':\n    access: $all\n    publish: $all\n    unpublish: $all\n    proxy: npmjs\nlog:\n  type: stdout\n  format: pretty\n  level: warn\n"
  ))
}

fn start_registry(root: &Path, config: &Path, log_path: &Path, port: u16) -> Result<Child> {
  let log = File::create(log_path)?;
  let error_log = log.try_clone()?;
  let clean_environment = std::env::vars_os().filter(|(key, _)| key != "NODE_OPTIONS");
  Command::new("nub")
    .args([
      OsStr::new("exec"),
      OsStr::new("verdaccio"),
      OsStr::new("--config"),
      config.as_os_str(),
      OsStr::new("--listen"),
      OsStr::new(&format!("127.0.0.1:{port}")),
    ])
    .current_dir(root)
    .env_clear()
    .envs(clean_environment)
    .stdin(Stdio::null())
    .stdout(Stdio::from(log))
    .stderr(Stdio::from(error_log))
    .spawn()
    .context("failed to start the local Verdaccio registry")
}

fn wait_for_registry(port: u16, log_path: &Path) -> Result<()> {
  for attempt in 0..=80 {
    if http_request(port, "GET", "/-/ping", None).is_ok_and(|response| response.is_success()) {
      return Ok(());
    }
    if attempt == 80 {
      let errors = fs::read_to_string(log_path).unwrap_or_default();
      bail!("Local registry did not start.\n{errors}");
    }
    std::thread::sleep(Duration::from_millis(100));
  }
  unreachable!()
}

fn create_registry_user(port: u16) -> Result<String> {
  let body = serde_json::to_vec(&json!({
    "name": REGISTRY_USER,
    "password": REGISTRY_PASSWORD,
    "email": "local-smoke@arcantry.invalid",
    "type": "user",
    "roles": [],
  }))?;
  let response = http_request(
    port,
    "PUT",
    &format!("/-/user/org.couchdb.user:{REGISTRY_USER}"),
    Some(&body),
  )?;
  if !response.is_success() {
    bail!(
      "Local registry user creation failed: {}",
      String::from_utf8_lossy(&response.body)
    );
  }
  let user: Value = serde_json::from_slice(&response.body)?;
  user
    .get("token")
    .and_then(Value::as_str)
    .map(str::to_owned)
    .context("Local registry returned no authentication token.")
}

struct HttpResponse {
  status: u16,
  body: Vec<u8>,
}

impl HttpResponse {
  fn is_success(&self) -> bool {
    (200..300).contains(&self.status)
  }
}

fn http_request(port: u16, method: &str, path: &str, body: Option<&[u8]>) -> Result<HttpResponse> {
  let mut stream = TcpStream::connect(("127.0.0.1", port))?;
  stream.set_read_timeout(Some(Duration::from_secs(2)))?;
  stream.set_write_timeout(Some(Duration::from_secs(2)))?;
  let body = body.unwrap_or_default();
  write!(
    stream,
    "{method} {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n",
    body.len()
  )?;
  stream.write_all(body)?;
  let mut response = Vec::new();
  stream.read_to_end(&mut response)?;
  parse_http_response(&response)
}

fn parse_http_response(response: &[u8]) -> Result<HttpResponse> {
  let boundary = response
    .windows(4)
    .position(|window| window == b"\r\n\r\n")
    .context("local registry returned a malformed HTTP response")?;
  let headers = std::str::from_utf8(&response[..boundary])?;
  let status = headers
    .lines()
    .next()
    .and_then(|line| line.split_whitespace().nth(1))
    .context("local registry response has no status")?
    .parse::<u16>()?;
  let body = &response[boundary + 4..];
  let body = if headers
    .to_ascii_lowercase()
    .contains("transfer-encoding: chunked")
  {
    decode_chunked(body)?
  } else {
    body.to_vec()
  };
  Ok(HttpResponse { status, body })
}

fn decode_chunked(mut body: &[u8]) -> Result<Vec<u8>> {
  let mut decoded = Vec::new();
  loop {
    let line_end = body
      .windows(2)
      .position(|window| window == b"\r\n")
      .context("malformed chunked response")?;
    let size = usize::from_str_radix(std::str::from_utf8(&body[..line_end])?.trim(), 16)?;
    body = &body[line_end + 2..];
    if size == 0 {
      return Ok(decoded);
    }
    if body.len() < size + 2 || &body[size..size + 2] != b"\r\n" {
      bail!("malformed chunked response body");
    }
    decoded.extend_from_slice(&body[..size]);
    body = &body[size + 2..];
  }
}

fn npm_archives(root: &Path) -> Result<Vec<PathBuf>> {
  let mut archives = fs::read_dir(root)?
    .filter_map(|entry| entry.ok())
    .map(|entry| entry.path())
    .filter(|path| path.extension() == Some(OsStr::new("tgz")))
    .collect::<Vec<_>>();
  archives.sort();
  if archives.len() != 7 {
    bail!("Expected seven npm archives, received {}.", archives.len());
  }
  Ok(archives)
}

fn package_version(root: &Path) -> Result<String> {
  let package: Value =
    serde_json::from_slice(&fs::read(root.join("packages/arcantry/package.json"))?)?;
  package
    .get("version")
    .and_then(Value::as_str)
    .map(str::to_owned)
    .context("main npm package has no version")
}

fn install_manifest(os: &str, cpu: &str, platform: &str, version: &str) -> Result<String> {
  let mut dependencies = Map::new();
  dependencies.insert("arcantry".to_owned(), Value::String(version.to_owned()));
  dependencies.insert(platform.to_owned(), Value::String(version.to_owned()));
  Ok(format!(
    "{}\n",
    serde_json::to_string_pretty(&json!({
      "name": format!("arcantry-registry-{os}-{cpu}"),
      "private": true,
      "dependencies": dependencies,
    }))?
  ))
}

fn registry_runners() -> Vec<(&'static str, Vec<&'static OsStr>)> {
  vec![
    (
      "npm",
      vec![
        OsStr::new("exec"),
        OsStr::new("--offline"),
        OsStr::new("--"),
        OsStr::new("arcantry"),
        OsStr::new("--version"),
      ],
    ),
    (
      "npx",
      vec![
        OsStr::new("--offline"),
        OsStr::new("arcantry"),
        OsStr::new("--version"),
      ],
    ),
    (
      "pnpm",
      vec![
        OsStr::new("exec"),
        OsStr::new("arcantry"),
        OsStr::new("--version"),
      ],
    ),
    (
      "bun",
      vec![
        OsStr::new("run"),
        OsStr::new("arcantry"),
        OsStr::new("--version"),
      ],
    ),
    (
      "nub",
      vec![
        OsStr::new("exec"),
        OsStr::new("arcantry"),
        OsStr::new("--version"),
      ],
    ),
  ]
}

fn run<I, S>(
  command: impl AsRef<OsStr>,
  arguments: I,
  cwd: &Path,
  user_config: Option<&Path>,
) -> Result<Vec<u8>>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let clean_environment = std::env::vars_os().filter(|(key, _)| key != "NODE_OPTIONS");
  let program = platform_program(command.as_ref());
  let mut process = Command::new(&program);
  process
    .args(arguments)
    .current_dir(cwd)
    .env_clear()
    .envs(clean_environment)
    .stdin(Stdio::null());
  if let Some(user_config) = user_config {
    process.env("NPM_CONFIG_USERCONFIG", user_config);
  }
  let output = process.output().with_context(|| {
    format!(
      "failed to execute registry smoke command {} in {}",
      program.to_string_lossy(),
      cwd.display()
    )
  })?;
  if !output.status.success() {
    bail!(
      "registry smoke command exited with {}: {}",
      output.status,
      String::from_utf8_lossy(&output.stderr).trim()
    );
  }
  Ok(output.stdout)
}

fn platform_program(command: &OsStr) -> OsString {
  #[cfg(windows)]
  if ["npm", "npx", "pnpm"]
    .iter()
    .any(|candidate| command == OsStr::new(candidate))
  {
    let mut program = command.to_os_string();
    program.push(".cmd");
    return program;
  }
  command.to_os_string()
}

fn stop_registry(registry: &mut Child) -> Result<()> {
  if registry.try_wait()?.is_some() {
    return Ok(());
  }
  #[cfg(windows)]
  {
    let killed_tree = Command::new("taskkill.exe")
      .args(["/pid", &registry.id().to_string(), "/t", "/f"])
      .stdin(Stdio::null())
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .status()
      .is_ok_and(|status| status.success());
    if !killed_tree {
      let _ = registry.kill();
    }
  }
  #[cfg(not(windows))]
  let _ = registry.kill();
  registry.wait()?;
  Ok(())
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
  fn writes_a_private_local_registry_configuration() {
    let config = registry_config(Path::new("registry-work")).unwrap();
    assert!(config.contains("url: https://registry.npmjs.org/"));
    assert!(config.contains("publish: $all"));
    assert!(config.contains("max_users: 1000"));
  }

  #[test]
  fn writes_target_specific_install_manifests() {
    let manifest = install_manifest("linux", "x64", "@arcantry/cli-linux-x64", "1.0.0").unwrap();
    let value: Value = serde_json::from_str(&manifest).unwrap();
    assert_eq!(value["name"], "arcantry-registry-linux-x64");
    assert_eq!(value["dependencies"]["arcantry"], "1.0.0");
    assert_eq!(value["dependencies"]["@arcantry/cli-linux-x64"], "1.0.0");
  }

  #[test]
  fn parses_content_length_and_chunked_registry_responses() {
    let regular = parse_http_response(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n{}").unwrap();
    assert!(regular.is_success());
    assert_eq!(regular.body, b"{}");
    let chunked = parse_http_response(
      b"HTTP/1.1 201 Created\r\nTransfer-Encoding: chunked\r\n\r\n7\r\n{\"a\":1}\r\n0\r\n\r\n",
    )
    .unwrap();
    assert!(chunked.is_success());
    assert_eq!(chunked.body, br#"{"a":1}"#);
  }
}
