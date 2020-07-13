// Based on:
// https://github.com/sfackler/cargo-tree/blob/4abcac4ddfb69f6dfde1af0164f2f0ee0e230336/src/metadata.rs

use anyhow::{anyhow, Context, Error};
use cargo_metadata::Metadata;
use std::env;
use std::ffi::OsString;
use std::process::{Command, Stdio};

fn check_arg(name: &str) -> Option<String> {
    let mut args = env::args().skip_while(|val| !val.starts_with(&name));
    match args.next() {
        // Get `bar` from `--foo bar`:
        Some(ref v) if v == name => Some(args.next().unwrap()),
        // Get `bar` from `--foo=bar`:
        Some(v) => Some(v.trim_start_matches(&format!("{}=", name)).to_owned()),
        None => None,
    }
}

pub fn get_metadata() -> Result<Metadata, Error> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));

    let mut command = Command::new(cargo);
    command.arg("metadata");
    command.arg("--format-version").arg("1");
    command
        .arg("--manifest-path")
        .arg(format!("{}/Cargo.toml", get_manifest_dir()?));

    // Map from rustc arguments to Cargo arguments.
    match check_arg("--target") {
        Some(v) => {
            command.arg("--filter-platform").arg(v);
        }
        _ => {
            command.arg("--filter-platform").arg(default_target()?);
        }
    };

    let output = output(&mut command, "cargo metadata")?;

    serde_json::from_str(&output).context("error parsing cargo metadata output")
}

pub fn get_manifest_dir() -> Result<String, Error> {
    match env::var("CARGO_MANIFEST_DIR") {
        Ok(path) => Ok(path),
        Err(e) => Err(Error::new(e)),
    }
}

pub fn default_target() -> Result<String, Error> {
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let output = output(Command::new(rustc).arg("-Vv"), "rustc")?;

    for line in output.lines() {
        let prefix = "host: ";
        if line.starts_with(prefix) {
            return Ok(line[prefix.len()..].trim().to_string());
        }
    }

    Err(anyhow!("host missing from rustc output"))
}

fn output(command: &mut Command, job: &str) -> Result<String, Error> {
    let output = command
        .stderr(Stdio::inherit())
        .output()
        .with_context(|| format!("error running {}", job))?;

    if !output.status.success() {
        return Err(anyhow!("{} returned {}", job, output.status));
    }

    String::from_utf8(output.stdout).with_context(|| format!("error parsing {} output", job))
}
