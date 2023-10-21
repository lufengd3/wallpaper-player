// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

pub use anyhow::Result;

mod build;
mod dev;
mod helpers;
mod icon;
mod info;
mod init;
mod interface;
mod plugin;
mod signer;

use clap::{ArgAction, CommandFactory, FromArgMatches, Parser, Subcommand};
use env_logger::fmt::Color;
use env_logger::Builder;
use log::{debug, log_enabled, Level};
use serde::Deserialize;
use std::io::{BufReader, Write};
use std::process::{exit, Command, ExitStatus, Output, Stdio};
use std::{
  ffi::OsString,
  sync::{Arc, Mutex},
};

#[derive(Deserialize)]
pub struct VersionMetadata {
  tauri: String,
  #[serde(rename = "tauri-build")]
  tauri_build: String,
}

#[derive(Deserialize)]
pub struct PackageJson {
  name: Option<String>,
  version: Option<String>,
  product_name: Option<String>,
}

#[derive(Parser)]
#[clap(
  author,
  version,
  about,
  bin_name("cargo-tauri"),
  subcommand_required(true),
  arg_required_else_help(true),
  propagate_version(true),
  no_binary_name(true)
)]
struct Cli {
  /// Enables verbose logging
  #[clap(short, long, global = true, action = ArgAction::Count)]
  verbose: u8,
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Build(build::Options),
  Dev(dev::Options),
  Icon(icon::Options),
  Info(info::Options),
  Init(init::Options),
  Plugin(plugin::Cli),
  Signer(signer::Cli),
}

fn format_error<I: CommandFactory>(err: clap::Error) -> clap::Error {
  let mut app = I::command();
  err.format(&mut app)
}

/// Run the Tauri CLI with the passed arguments, exiting if an error occurs.
///
/// The passed arguments should have the binary argument(s) stripped out before being passed.
///
/// e.g.
/// 1. `tauri-cli 1 2 3` -> `1 2 3`
/// 2. `cargo tauri 1 2 3` -> `1 2 3`
/// 3. `node tauri.js 1 2 3` -> `1 2 3`
///
/// The passed `bin_name` parameter should be how you want the help messages to display the command.
/// This defaults to `cargo-tauri`, but should be set to how the program was called, such as
/// `cargo tauri`.
pub fn run<I, A>(args: I, bin_name: Option<String>)
where
  I: IntoIterator<Item = A>,
  A: Into<OsString> + Clone,
{
  if let Err(e) = try_run(args, bin_name) {
    log::error!("{:#}", e);
    exit(1);
  }
}

/// Run the Tauri CLI with the passed arguments.
///
/// It is similar to [`run`], but instead of exiting on an error, it returns a result.
pub fn try_run<I, A>(args: I, bin_name: Option<String>) -> Result<()>
where
  I: IntoIterator<Item = A>,
  A: Into<OsString> + Clone,
{
  let matches = match bin_name {
    Some(bin_name) => Cli::command().bin_name(bin_name),
    None => Cli::command(),
  }
  .get_matches_from(args);

  let res = Cli::from_arg_matches(&matches).map_err(format_error::<Cli>);
  let cli = match res {
    Ok(s) => s,
    Err(e) => e.exit(),
  };

  let mut builder = Builder::from_default_env();
  let init_res = builder
    .format_indent(Some(12))
    .filter(None, verbosity_level(cli.verbose).to_level_filter())
    .format(|f, record| {
      let mut is_command_output = false;
      if let Some(action) = record.key_values().get("action".into()) {
        let action = action.to_str().unwrap();
        is_command_output = action == "stdout" || action == "stderr";
        if !is_command_output {
          let mut action_style = f.style();
          action_style.set_color(Color::Green).set_bold(true);

          write!(f, "{:>12} ", action_style.value(action))?;
        }
      } else {
        let mut level_style = f.default_level_style(record.level());
        level_style.set_bold(true);

        write!(
          f,
          "{:>12} ",
          level_style.value(prettyprint_level(record.level()))
        )?;
      }

      if !is_command_output && log_enabled!(Level::Debug) {
        let mut target_style = f.style();
        target_style.set_color(Color::Black);

        write!(f, "[{}] ", target_style.value(record.target()))?;
      }

      writeln!(f, "{}", record.args())
    })
    .try_init();

  if let Err(err) = init_res {
    eprintln!("Failed to attach logger: {}", err);
  }

  match cli.command {
    Commands::Build(options) => build::command(options)?,
    Commands::Dev(options) => dev::command(options)?,
    Commands::Icon(options) => icon::command(options)?,
    Commands::Info(options) => info::command(options)?,
    Commands::Init(options) => init::command(options)?,
    Commands::Plugin(cli) => plugin::command(cli)?,
    Commands::Signer(cli) => signer::command(cli)?,
  }

  Ok(())
}

/// This maps the occurrence of `--verbose` flags to the correct log level
fn verbosity_level(num: u8) -> Level {
  match num {
    0 => Level::Info,
    1 => Level::Debug,
    2.. => Level::Trace,
  }
}

/// The default string representation for `Level` is all uppercaps which doesn't mix well with the other printed actions.
fn prettyprint_level(lvl: Level) -> &'static str {
  match lvl {
    Level::Error => "Error",
    Level::Warn => "Warn",
    Level::Info => "Info",
    Level::Debug => "Debug",
    Level::Trace => "Trace",
  }
}

pub trait CommandExt {
  // The `pipe` function sets the stdout and stderr to properly
  // show the command output in the Node.js wrapper.
  fn piped(&mut self) -> std::io::Result<ExitStatus>;
  fn output_ok(&mut self) -> crate::Result<Output>;
}

impl CommandExt for Command {
  fn piped(&mut self) -> std::io::Result<ExitStatus> {
    self.stdout(os_pipe::dup_stdout()?);
    self.stderr(os_pipe::dup_stderr()?);
    let program = self.get_program().to_string_lossy().into_owned();
    debug!(action = "Running"; "Command `{} {}`", program, self.get_args().map(|arg| arg.to_string_lossy()).fold(String::new(), |acc, arg| format!("{} {}", acc, arg)));

    self.status().map_err(Into::into)
  }

  fn output_ok(&mut self) -> crate::Result<Output> {
    let program = self.get_program().to_string_lossy().into_owned();
    debug!(action = "Running"; "Command `{} {}`", program, self.get_args().map(|arg| arg.to_string_lossy()).fold(String::new(), |acc, arg| format!("{} {}", acc, arg)));

    self.stdout(Stdio::piped());
    self.stderr(Stdio::piped());

    let mut child = self.spawn()?;

    let mut stdout = child.stdout.take().map(BufReader::new).unwrap();
    let stdout_lines = Arc::new(Mutex::new(Vec::new()));
    let stdout_lines_ = stdout_lines.clone();
    std::thread::spawn(move || {
      let mut buf = Vec::new();
      let mut lines = stdout_lines_.lock().unwrap();
      loop {
        buf.clear();
        match tauri_utils::io::read_line(&mut stdout, &mut buf) {
          Ok(s) if s == 0 => break,
          _ => (),
        }
        debug!(action = "stdout"; "{}", String::from_utf8_lossy(&buf));
        lines.extend(buf.clone());
        lines.push(b'\n');
      }
    });

    let mut stderr = child.stderr.take().map(BufReader::new).unwrap();
    let stderr_lines = Arc::new(Mutex::new(Vec::new()));
    let stderr_lines_ = stderr_lines.clone();
    std::thread::spawn(move || {
      let mut buf = Vec::new();
      let mut lines = stderr_lines_.lock().unwrap();
      loop {
        buf.clear();
        match tauri_utils::io::read_line(&mut stderr, &mut buf) {
          Ok(s) if s == 0 => break,
          _ => (),
        }
        debug!(action = "stderr"; "{}", String::from_utf8_lossy(&buf));
        lines.extend(buf.clone());
        lines.push(b'\n');
      }
    });

    let status = child.wait()?;

    let output = Output {
      status,
      stdout: std::mem::take(&mut *stdout_lines.lock().unwrap()),
      stderr: std::mem::take(&mut *stderr_lines.lock().unwrap()),
    };

    if output.status.success() {
      Ok(output)
    } else {
      Err(anyhow::anyhow!("failed to run {}", program))
    }
  }
}
