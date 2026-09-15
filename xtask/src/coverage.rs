use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug, Deserialize)]
struct CoverageFloor {
  line: f64,
  branch: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CoveragePolicy {
  schema_version: u32,
  files: BTreeMap<String, CoverageFloor>,
  exclusions: BTreeMap<String, String>,
}

#[derive(Debug, PartialEq)]
struct CoverageSummary {
  path: String,
  line: CoverageMetric,
  branch: CoverageMetric,
}

#[derive(Debug, PartialEq)]
struct CoverageMetric {
  found: usize,
  hit: usize,
  percent: Option<f64>,
}

#[derive(Default)]
struct MutableCoverage {
  lines: BTreeMap<u32, u64>,
  branches: BTreeMap<String, u64>,
}

pub fn run(root: &Path) -> Result<()> {
  let root = fs::canonicalize(root).context("could not resolve the coverage workspace root")?;
  let mut coverage_environment = coverage_environment(&root)?;

  successful(
    Command::new("cargo")
      .args(["llvm-cov", "clean", "--workspace"])
      .current_dir(&root)
      .env_remove("CARGO_TARGET_DIR")
      .envs(&coverage_environment),
    "cargo llvm-cov clean --workspace",
  )?;
  successful(
    Command::new("cargo")
      .args(["build", "--workspace"])
      .current_dir(&root)
      .env_remove("CARGO_TARGET_DIR")
      .envs(&coverage_environment),
    "cargo build --workspace",
  )?;

  let binary = root.join(format!(
    "target/debug/arcantry{}",
    std::env::consts::EXE_SUFFIX
  ));
  coverage_environment.insert(
    OsString::from("ARCANTRY_BIN"),
    binary.as_os_str().to_owned(),
  );
  let fixture = tempfile::Builder::new()
    .prefix("arcantry-coverage-")
    .tempdir()?;
  successful(
    Command::new("git")
      .args([
        OsStr::new("init"),
        OsStr::new("--quiet"),
        fixture.path().as_os_str(),
      ])
      .current_dir(&root)
      .envs(&coverage_environment),
    "git init coverage fixture",
  )?;

  let scenarios = [
    (
      vec![
        OsString::from("--cwd"),
        fixture.path().as_os_str().to_owned(),
        OsString::from("repo"),
        OsString::from("inspect"),
        OsString::from("--json"),
      ],
      false,
    ),
    (
      vec![
        OsString::from("--cwd"),
        fixture.path().as_os_str().to_owned(),
        OsString::from("todo"),
        OsString::from("list"),
      ],
      false,
    ),
    (
      vec![
        OsString::from("--cwd"),
        fixture.path().as_os_str().to_owned(),
        OsString::from("release"),
        OsString::from("plan"),
      ],
      true,
    ),
    (
      vec![
        OsString::from("--cwd"),
        root.as_os_str().to_owned(),
        OsString::from("skills"),
        OsString::from("list"),
      ],
      false,
    ),
  ];
  for (arguments, allow_failure) in scenarios {
    instrumented_cli(
      &binary,
      &arguments,
      &root,
      &coverage_environment,
      allow_failure,
    )?;
  }

  successful(
    Command::new("cargo")
      .args(["test", "--workspace"])
      .current_dir(&root)
      .env_remove("CARGO_TARGET_DIR")
      .envs(&coverage_environment),
    "cargo test --workspace",
  )?;
  successful(
    Command::new("cargo")
      .args([
        "llvm-cov",
        "report",
        "--branch",
        "--include-build-script",
        "--lcov",
        "--output-path",
        "target/rust-coverage.lcov",
      ])
      .current_dir(&root)
      .env_remove("CARGO_TARGET_DIR")
      .envs(&coverage_environment),
    "cargo llvm-cov report",
  )?;
  verify(
    &root,
    Path::new("target/rust-coverage.lcov"),
    Path::new("contracts/rust-coverage-policy.json"),
  )
}

fn coverage_environment(root: &Path) -> Result<BTreeMap<OsString, OsString>> {
  let output = Command::new("cargo")
    .args(["llvm-cov", "show-env", "--branch"])
    .current_dir(root)
    .env_remove("CARGO_TARGET_DIR")
    .output()
    .context("cargo llvm-cov show-env could not start")?;
  if !output.status.success() {
    bail!(
      "cargo llvm-cov show-env failed: {}",
      String::from_utf8_lossy(&output.stderr).trim()
    );
  }
  Ok(parse_coverage_environment(&String::from_utf8(
    output.stdout,
  )?))
}

fn parse_coverage_environment(output: &str) -> BTreeMap<OsString, OsString> {
  output
    .lines()
    .filter_map(|line| {
      let (key, raw_value) = line.split_once('=')?;
      if key.is_empty() {
        return None;
      }
      let value = raw_value
        .strip_prefix('\'')
        .and_then(|value| value.strip_suffix('\''))
        .unwrap_or(raw_value);
      Some((OsString::from(key), OsString::from(value)))
    })
    .collect()
}

fn successful(command: &mut Command, description: &str) -> Result<()> {
  let status = command
    .status()
    .with_context(|| format!("{description} could not start"))?;
  if !status.success() {
    bail!("{description} exited with {status}");
  }
  Ok(())
}

fn instrumented_cli(
  binary: &Path,
  arguments: &[OsString],
  root: &Path,
  environment: &BTreeMap<OsString, OsString>,
  allow_failure: bool,
) -> Result<()> {
  let output = Command::new(binary)
    .args(arguments)
    .current_dir(root)
    .envs(environment)
    .output()
    .with_context(|| {
      format!(
        "instrumented Arcantry could not start at {}",
        binary.display()
      )
    })?;
  if !allow_failure && !output.status.success() {
    let arguments = arguments
      .iter()
      .map(|argument| argument.to_string_lossy())
      .collect::<Vec<_>>()
      .join(" ");
    bail!(
      "instrumented arcantry {arguments} exited with {}: {}",
      output.status,
      String::from_utf8_lossy(&output.stderr).trim()
    );
  }
  Ok(())
}

pub fn verify(root: &Path, report_path: &Path, policy_path: &Path) -> Result<()> {
  let report = fs::read_to_string(resolve(root, report_path))?;
  let policy =
    serde_json::from_str::<CoveragePolicy>(&fs::read_to_string(resolve(root, policy_path))?)?;
  let mut production_files = rust_files(root, &root.join("crates/arcantry-cli/src"))?;
  production_files.extend(rust_files(root, &root.join("crates/arcantry-core/src"))?);
  production_files.push("crates/arcantry-cli/build.rs".to_owned());

  for summary in verify_report(&report, &policy, &production_files)? {
    let line = summary.line.percent.unwrap_or(0.0);
    let branch = summary
      .branch
      .percent
      .map_or_else(|| "n/a".to_owned(), |value| format!("{value:.2}%"));
    println!("{}: lines {line:.2}%, branches {branch}", summary.path);
  }
  Ok(())
}

fn verify_report(
  report: &str,
  policy: &CoveragePolicy,
  production_files: &[String],
) -> Result<Vec<CoverageSummary>> {
  if policy.schema_version != 1 {
    bail!("Rust coverage policy schemaVersion must be 1.");
  }
  let files = production_files
    .iter()
    .map(|path| normalize(path))
    .collect::<BTreeSet<_>>();
  let policy_paths = policy
    .files
    .keys()
    .map(|path| normalize(path))
    .collect::<BTreeSet<_>>();
  let exclusion_paths = policy
    .exclusions
    .keys()
    .map(|path| normalize(path))
    .collect::<BTreeSet<_>>();

  if let Some(duplicate) = policy_paths.intersection(&exclusion_paths).next() {
    bail!("Rust coverage policy lists {duplicate} as both covered and excluded.");
  }
  let inventoried = policy_paths
    .union(&exclusion_paths)
    .cloned()
    .collect::<BTreeSet<_>>();
  let missing = files.difference(&inventoried).cloned().collect::<Vec<_>>();
  if !missing.is_empty() {
    bail!(
      "Rust coverage policy is missing production files: {}",
      missing.join(", ")
    );
  }
  let stale = inventoried.difference(&files).cloned().collect::<Vec<_>>();
  if !stale.is_empty() {
    bail!(
      "Rust coverage policy contains stale files: {}",
      stale.join(", ")
    );
  }
  for (path, reason) in &policy.exclusions {
    if reason.trim().is_empty() {
      bail!("Rust coverage exclusion {path} requires a reason.");
    }
  }

  let records = parse_lcov(report)?;
  let mut failures = Vec::new();
  let mut summaries = Vec::new();
  for (path, floor) in &policy.files {
    validate_floor(path, floor)?;
    let Some(record) = records.get(&normalize(path)) else {
      failures.push(format!("{path}: missing from coverage report"));
      continue;
    };
    let line = metric(record.lines.values().copied());
    let branch = metric(record.branches.values().copied());
    let line_percent = line.percent.unwrap_or(0.0);
    if line.found == 0 || line.hit == 0 {
      failures.push(format!("{path}: no executed line evidence"));
    }
    if line_percent + f64::EPSILON < floor.line {
      failures.push(format!(
        "{path}: line coverage {line_percent:.2}% is below {}%",
        floor.line
      ));
    }
    match (branch.percent, floor.branch) {
      (Some(_), None) => failures.push(format!(
        "{path}: branch records exist but the policy has no branch floor"
      )),
      (None, Some(_)) => failures.push(format!("{path}: branch floor requires branch records")),
      (Some(percent), Some(floor)) if percent + f64::EPSILON < floor => failures.push(format!(
        "{path}: branch coverage {percent:.2}% is below {floor}%"
      )),
      _ => {}
    }
    summaries.push(CoverageSummary {
      path: path.clone(),
      line,
      branch,
    });
  }
  if !failures.is_empty() {
    bail!("Rust coverage policy failed:\n- {}", failures.join("\n- "));
  }
  summaries.sort_by(|left, right| left.path.cmp(&right.path));
  Ok(summaries)
}

fn validate_floor(path: &str, floor: &CoverageFloor) -> Result<()> {
  for (kind, value) in [("line", Some(floor.line)), ("branch", floor.branch)] {
    if value.is_some_and(|value| !value.is_finite() || !(0.0..=100.0).contains(&value)) {
      bail!("{path}: {kind} floor must be between 0 and 100.");
    }
  }
  Ok(())
}

fn parse_lcov(report: &str) -> Result<BTreeMap<String, MutableCoverage>> {
  let mut records = BTreeMap::<String, MutableCoverage>::new();
  let mut current = None::<String>;
  for line in report.lines() {
    if let Some(path) = line.strip_prefix("SF:") {
      let path = normalize_source_path(path);
      records.entry(path.clone()).or_default();
      current = Some(path);
    } else if let (Some(path), Some(value)) = (&current, line.strip_prefix("DA:")) {
      let mut fields = value.split(',');
      let line_number = fields
        .next()
        .context("LCOV DA record is missing a line number")?
        .parse()?;
      let count = fields
        .next()
        .context("LCOV DA record is missing an execution count")?
        .parse()?;
      let record = records.get_mut(path).expect("current LCOV record exists");
      record
        .lines
        .entry(line_number)
        .and_modify(|existing| *existing = (*existing).max(count))
        .or_insert(count);
    } else if let (Some(path), Some(value)) = (&current, line.strip_prefix("BRDA:")) {
      let fields = value.split(',').collect::<Vec<_>>();
      if fields.len() < 4 {
        bail!("LCOV BRDA record is incomplete: {line}");
      }
      let taken = if fields[3] == "-" {
        0
      } else {
        fields[3].parse()?
      };
      let key = format!("{},{},{}", fields[0], fields[1], fields[2]);
      let record = records.get_mut(path).expect("current LCOV record exists");
      record
        .branches
        .entry(key)
        .and_modify(|existing| *existing = (*existing).max(taken))
        .or_insert(taken);
    } else if line == "end_of_record" {
      current = None;
    }
  }
  Ok(records)
}

fn metric(values: impl Iterator<Item = u64>) -> CoverageMetric {
  let values = values.collect::<Vec<_>>();
  let found = values.len();
  let hit = values.iter().filter(|count| **count > 0).count();
  CoverageMetric {
    found,
    hit,
    percent: (found > 0).then(|| hit as f64 / found as f64 * 100.0),
  }
}

fn normalize(path: &str) -> String {
  let normalized = path.replace('\\', "/");
  normalized
    .strip_prefix("./")
    .unwrap_or(&normalized)
    .to_owned()
}

fn normalize_source_path(path: &str) -> String {
  let normalized = normalize(path);
  for marker in ["/crates/", "/xtask/"] {
    if let Some(index) = normalized.rfind(marker) {
      return normalized[index + 1..].to_owned();
    }
  }
  normalized
}

fn rust_files(root: &Path, directory: &Path) -> Result<Vec<String>> {
  let mut found = Vec::new();
  for entry in fs::read_dir(directory)
    .with_context(|| format!("could not inventory Rust files in {}", directory.display()))?
  {
    let path = entry?.path();
    if path.is_dir() {
      found.extend(rust_files(root, &path)?);
    } else if path.extension().is_some_and(|extension| extension == "rs") {
      found.push(normalize(
        path.strip_prefix(root)?.to_string_lossy().as_ref(),
      ));
    }
  }
  Ok(found)
}

fn resolve(root: &Path, path: &Path) -> PathBuf {
  if path.is_absolute() {
    path.to_owned()
  } else {
    root.join(path)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn policy() -> CoveragePolicy {
    CoveragePolicy {
      schema_version: 1,
      files: BTreeMap::from([(
        "crates/example/src/main.rs".to_owned(),
        CoverageFloor {
          line: 50.0,
          branch: Some(50.0),
        },
      )]),
      exclusions: BTreeMap::from([(
        "crates/example/src/lib.rs".to_owned(),
        "Module exports only.".to_owned(),
      )]),
    }
  }

  const REPORT: &str = "SF:/workspace/crates/example/src/main.rs\nDA:1,1\nDA:2,0\nBRDA:1,0,0,1\nBRDA:1,0,1,0\nend_of_record\n";

  #[test]
  fn parses_llvm_cov_environment_assignments() {
    let environment = parse_coverage_environment("QUOTED='one two'\nPLAIN=value=tail\ninvalid\n");
    assert_eq!(
      environment.get(OsStr::new("QUOTED")),
      Some(&OsString::from("one two"))
    );
    assert_eq!(
      environment.get(OsStr::new("PLAIN")),
      Some(&OsString::from("value=tail"))
    );
    assert_eq!(environment.len(), 2);
  }

  #[test]
  fn accepts_per_file_line_and_branch_evidence_at_the_reviewed_floors() {
    let summaries = verify_report(
      REPORT,
      &policy(),
      &[
        "crates/example/src/main.rs".to_owned(),
        "crates/example/src/lib.rs".to_owned(),
      ],
    )
    .unwrap();
    assert_eq!(summaries[0].line.percent, Some(50.0));
    assert_eq!(summaries[0].branch.percent, Some(50.0));
  }

  #[test]
  fn rejects_a_new_production_file_absent_from_the_policy() {
    let error = verify_report(
      REPORT,
      &policy(),
      &[
        "crates/example/src/main.rs".to_owned(),
        "crates/example/src/lib.rs".to_owned(),
        "crates/example/src/new.rs".to_owned(),
      ],
    )
    .unwrap_err();
    assert!(
      error
        .to_string()
        .contains("missing production files: crates/example/src/new.rs")
    );
  }

  #[test]
  fn rejects_line_coverage_below_a_file_floor() {
    let mut policy = policy();
    policy
      .files
      .get_mut("crates/example/src/main.rs")
      .unwrap()
      .line = 51.0;
    let error = verify_report(
      REPORT,
      &policy,
      &[
        "crates/example/src/main.rs".to_owned(),
        "crates/example/src/lib.rs".to_owned(),
      ],
    )
    .unwrap_err();
    assert!(
      error
        .to_string()
        .contains("line coverage 50.00% is below 51%")
    );
  }

  #[test]
  fn rejects_branch_coverage_below_a_file_floor() {
    let mut policy = policy();
    policy
      .files
      .get_mut("crates/example/src/main.rs")
      .unwrap()
      .branch = Some(51.0);
    let error = verify_report(
      REPORT,
      &policy,
      &[
        "crates/example/src/main.rs".to_owned(),
        "crates/example/src/lib.rs".to_owned(),
      ],
    )
    .unwrap_err();
    assert!(
      error
        .to_string()
        .contains("branch coverage 50.00% is below 51%")
    );
  }
}
