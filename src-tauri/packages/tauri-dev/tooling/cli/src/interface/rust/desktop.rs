use super::{AppSettings, DevChild, ExitReason, Options, RustAppSettings, Target};
use crate::CommandExt;

use anyhow::Context;
#[cfg(target_os = "linux")]
use heck::ToKebabCase;
use shared_child::SharedChild;
use std::{
  fs::rename,
  io::{BufReader, ErrorKind, Write},
  path::{Path, PathBuf},
  process::{Command, ExitStatus, Stdio},
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
  },
};

pub fn run_dev<F: Fn(ExitStatus, ExitReason) + Send + Sync + 'static>(
  options: Options,
  run_args: Vec<String>,
  available_targets: &mut Option<Vec<Target>>,
  config_features: Vec<String>,
  app_settings: &RustAppSettings,
  product_name: Option<String>,
  on_exit: F,
) -> crate::Result<DevChild> {
  let bin_path = app_settings.app_binary_path(&options)?;

  let manually_killed_app = Arc::new(AtomicBool::default());
  let manually_killed_app_ = manually_killed_app.clone();
  let app_child = Arc::new(Mutex::new(None));
  let app_child_ = app_child.clone();

  let build_child = build_dev_app(
    options,
    available_targets,
    config_features,
    move |status, reason| {
      if status.success() {
        let bin_path =
          rename_app(&bin_path, product_name.as_deref()).expect("failed to rename app");
        let mut app = Command::new(bin_path);
        app.stdout(os_pipe::dup_stdout().unwrap());
        app.stderr(os_pipe::dup_stderr().unwrap());
        app.args(run_args);
        let app_child = Arc::new(SharedChild::spawn(&mut app).unwrap());
        let app_child_t = app_child.clone();
        std::thread::spawn(move || {
          let status = app_child_t.wait().expect("failed to wait on app");
          on_exit(
            status,
            if manually_killed_app_.load(Ordering::Relaxed) {
              ExitReason::TriggeredKill
            } else {
              ExitReason::NormalExit
            },
          );
        });

        app_child_.lock().unwrap().replace(app_child);
      } else {
        on_exit(
          status,
          if manually_killed_app_.load(Ordering::Relaxed) {
            ExitReason::TriggeredKill
          } else {
            reason
          },
        );
      }
    },
  )?;

  Ok(DevChild {
    manually_killed_app,
    build_child,
    app_child,
  })
}

pub fn build(
  options: Options,
  app_settings: &RustAppSettings,
  product_name: Option<String>,
  available_targets: &mut Option<Vec<Target>>,
  config_features: Vec<String>,
) -> crate::Result<()> {
  let bin_path = app_settings.app_binary_path(&options)?;
  let out_dir = bin_path.parent().unwrap();

  let bin_name = bin_path.file_stem().unwrap();

  if !std::env::var("STATIC_VCRUNTIME").map_or(false, |v| v == "false") {
    std::env::set_var("STATIC_VCRUNTIME", "true");
  }

  if options.target == Some("universal-apple-darwin".into()) {
    std::fs::create_dir_all(out_dir).with_context(|| "failed to create project out directory")?;

    let mut lipo_cmd = Command::new("lipo");
    lipo_cmd
      .arg("-create")
      .arg("-output")
      .arg(out_dir.join(bin_name));
    for triple in ["aarch64-apple-darwin", "x86_64-apple-darwin"] {
      let mut options = options.clone();
      options.target.replace(triple.into());

      let triple_out_dir = app_settings
        .out_dir(Some(triple.into()), options.debug)
        .with_context(|| format!("failed to get {} out dir", triple))?;

      build_production_app(options, available_targets, config_features.clone())
        .with_context(|| format!("failed to build {} binary", triple))?;

      lipo_cmd.arg(triple_out_dir.join(bin_name));
    }

    let lipo_status = lipo_cmd.output_ok()?.status;
    if !lipo_status.success() {
      return Err(anyhow::anyhow!(format!(
        "Result of `lipo` command was unsuccessful: {}. (Is `lipo` installed?)",
        lipo_status
      )));
    }
  } else {
    build_production_app(options, available_targets, config_features)
      .with_context(|| "failed to build app")?;
  }

  rename_app(&bin_path, product_name.as_deref())?;

  Ok(())
}

fn build_dev_app<F: FnOnce(ExitStatus, ExitReason) + Send + 'static>(
  options: Options,
  available_targets: &mut Option<Vec<Target>>,
  config_features: Vec<String>,
  on_exit: F,
) -> crate::Result<Arc<SharedChild>> {
  let mut build_cmd = build_command(options, available_targets, config_features)?;
  let runner = build_cmd.get_program().to_string_lossy().into_owned();
  build_cmd
    .env(
      "CARGO_TERM_PROGRESS_WIDTH",
      terminal::stderr_width()
        .map(|width| {
          if cfg!(windows) {
            std::cmp::min(60, width)
          } else {
            width
          }
        })
        .unwrap_or(if cfg!(windows) { 60 } else { 80 })
        .to_string(),
    )
    .env("CARGO_TERM_PROGRESS_WHEN", "always");
  build_cmd.arg("--color");
  build_cmd.arg("always");

  build_cmd.stdout(os_pipe::dup_stdout()?);
  build_cmd.stderr(Stdio::piped());

  let build_child = match SharedChild::spawn(&mut build_cmd) {
    Ok(c) => Ok(c),
    Err(e) if e.kind() == ErrorKind::NotFound => Err(anyhow::anyhow!(
      "`{}` command not found.{}",
      runner,
      if runner == "cargo" {
        " Please follow the Tauri setup guide: https://tauri.app/v1/guides/getting-started/prerequisites"
      } else {
        ""
      }
    )),
    Err(e) => Err(e.into()),
  }?;
  let build_child = Arc::new(build_child);
  let build_child_stderr = build_child.take_stderr().unwrap();
  let mut stderr = BufReader::new(build_child_stderr);
  let stderr_lines = Arc::new(Mutex::new(Vec::new()));
  let stderr_lines_ = stderr_lines.clone();
  std::thread::spawn(move || {
    let mut buf = Vec::new();
    let mut lines = stderr_lines_.lock().unwrap();
    let mut io_stderr = std::io::stderr();
    loop {
      buf.clear();
      match tauri_utils::io::read_line(&mut stderr, &mut buf) {
        Ok(s) if s == 0 => break,
        _ => (),
      }
      let _ = io_stderr.write_all(&buf);
      if !buf.ends_with(&[b'\r']) {
        let _ = io_stderr.write_all(b"\n");
      }
      lines.push(String::from_utf8_lossy(&buf).into_owned());
    }
  });

  let build_child_ = build_child.clone();
  std::thread::spawn(move || {
    let status = build_child_.wait().expect("failed to wait on build");

    if status.success() {
      on_exit(status, ExitReason::NormalExit);
    } else {
      let is_cargo_compile_error = stderr_lines
        .lock()
        .unwrap()
        .last()
        .map(|l| l.contains("could not compile"))
        .unwrap_or_default();
      stderr_lines.lock().unwrap().clear();

      on_exit(
        status,
        if status.code() == Some(101) && is_cargo_compile_error {
          ExitReason::CompilationFailed
        } else {
          ExitReason::NormalExit
        },
      );
    }
  });

  Ok(build_child)
}

fn build_production_app(
  options: Options,
  available_targets: &mut Option<Vec<Target>>,
  config_features: Vec<String>,
) -> crate::Result<()> {
  let mut build_cmd = build_command(options, available_targets, config_features)?;
  let runner = build_cmd.get_program().to_string_lossy().into_owned();
  match build_cmd.piped() {
    Ok(status) if status.success() => Ok(()),
    Ok(_) => Err(anyhow::anyhow!("failed to build app")),
    Err(e) if e.kind() == ErrorKind::NotFound => Err(anyhow::anyhow!(
      "`{}` command not found.{}",
      runner,
      if runner == "cargo" {
        " Please follow the Tauri setup guide: https://tauri.app/v1/guides/getting-started/prerequisites"
      } else {
        ""
      }
    )),
    Err(e) => Err(e.into()),
  }
}

fn build_command(
  options: Options,
  available_targets: &mut Option<Vec<Target>>,
  config_features: Vec<String>,
) -> crate::Result<Command> {
  let runner = options.runner.unwrap_or_else(|| "cargo".into());

  if let Some(target) = &options.target {
    if available_targets.is_none() {
      *available_targets = fetch_available_targets();
    }
    validate_target(available_targets, target)?;
  }

  let mut args = Vec::new();
  if !options.args.is_empty() {
    args.extend(options.args);
  }

  let mut features = config_features;
  if let Some(f) = options.features {
    features.extend(f);
  }
  if !features.is_empty() {
    args.push("--features".into());
    args.push(features.join(","));
  }

  if !options.debug {
    args.push("--release".into());
  }

  if let Some(target) = options.target {
    args.push("--target".into());
    args.push(target);
  }

  let mut build_cmd = Command::new(&runner);
  build_cmd.arg("build");
  build_cmd.args(args);

  Ok(build_cmd)
}

fn fetch_available_targets() -> Option<Vec<Target>> {
  if let Ok(output) = Command::new("rustup").args(["target", "list"]).output() {
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    Some(
      stdout
        .split('\n')
        .map(|t| {
          let mut s = t.split(' ');
          let name = s.next().unwrap().to_string();
          let installed = s.next().map(|v| v == "(installed)").unwrap_or_default();
          Target { name, installed }
        })
        .filter(|t| !t.name.is_empty())
        .collect(),
    )
  } else {
    None
  }
}

fn validate_target(available_targets: &Option<Vec<Target>>, target: &str) -> crate::Result<()> {
  if let Some(available_targets) = available_targets {
    if let Some(target) = available_targets.iter().find(|t| t.name == target) {
      if !target.installed {
        anyhow::bail!(
            "Target {target} is not installed (installed targets: {installed}). Please run `rustup target add {target}`.",
            target = target.name,
            installed = available_targets.iter().filter(|t| t.installed).map(|t| t.name.as_str()).collect::<Vec<&str>>().join(", ")
          );
      }
    }
    if !available_targets.iter().any(|t| t.name == target) {
      anyhow::bail!("Target {target} does not exist. Please run `rustup target list` to see the available targets.", target = target);
    }
  }
  Ok(())
}

fn rename_app(bin_path: &Path, product_name: Option<&str>) -> crate::Result<PathBuf> {
  if let Some(product_name) = product_name {
    #[cfg(target_os = "linux")]
    let product_name = product_name.to_kebab_case();

    let product_path = bin_path
      .parent()
      .unwrap()
      .join(&product_name)
      .with_extension(bin_path.extension().unwrap_or_default());

    rename(bin_path, &product_path).with_context(|| {
      format!(
        "failed to rename `{}` to `{}`",
        bin_path.display(),
        product_path.display(),
      )
    })?;
    Ok(product_path)
  } else {
    Ok(bin_path.to_path_buf())
  }
}

// taken from https://github.com/rust-lang/cargo/blob/78b10d4e611ab0721fc3aeaf0edd5dd8f4fdc372/src/cargo/core/shell.rs#L514
#[cfg(unix)]
mod terminal {
  use std::mem;

  pub fn stderr_width() -> Option<usize> {
    unsafe {
      let mut winsize: libc::winsize = mem::zeroed();
      // The .into() here is needed for FreeBSD which defines TIOCGWINSZ
      // as c_uint but ioctl wants c_ulong.
      #[allow(clippy::useless_conversion)]
      if libc::ioctl(libc::STDERR_FILENO, libc::TIOCGWINSZ.into(), &mut winsize) < 0 {
        return None;
      }
      if winsize.ws_col > 0 {
        Some(winsize.ws_col as usize)
      } else {
        None
      }
    }
  }
}

// taken from https://github.com/rust-lang/cargo/blob/78b10d4e611ab0721fc3aeaf0edd5dd8f4fdc372/src/cargo/core/shell.rs#L543
#[cfg(windows)]
mod terminal {
  use std::{cmp, mem, ptr};
  use winapi::um::fileapi::*;
  use winapi::um::handleapi::*;
  use winapi::um::processenv::*;
  use winapi::um::winbase::*;
  use winapi::um::wincon::*;
  use winapi::um::winnt::*;

  pub fn stderr_width() -> Option<usize> {
    unsafe {
      let stdout = GetStdHandle(STD_ERROR_HANDLE);
      let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
      if GetConsoleScreenBufferInfo(stdout, &mut csbi) != 0 {
        return Some((csbi.srWindow.Right - csbi.srWindow.Left) as usize);
      }

      // On mintty/msys/cygwin based terminals, the above fails with
      // INVALID_HANDLE_VALUE. Use an alternate method which works
      // in that case as well.
      let h = CreateFileA(
        "CONOUT$\0".as_ptr() as *const CHAR,
        GENERIC_READ | GENERIC_WRITE,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        ptr::null_mut(),
        OPEN_EXISTING,
        0,
        ptr::null_mut(),
      );
      if h == INVALID_HANDLE_VALUE {
        return None;
      }

      let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
      let rc = GetConsoleScreenBufferInfo(h, &mut csbi);
      CloseHandle(h);
      if rc != 0 {
        let width = (csbi.srWindow.Right - csbi.srWindow.Left) as usize;
        // Unfortunately cygwin/mintty does not set the size of the
        // backing console to match the actual window size. This
        // always reports a size of 80 or 120 (not sure what
        // determines that). Use a conservative max of 60 which should
        // work in most circumstances. ConEmu does some magic to
        // resize the console correctly, but there's no reasonable way
        // to detect which kind of terminal we are running in, or if
        // GetConsoleScreenBufferInfo returns accurate information.
        return Some(cmp::min(60, width));
      }

      None
    }
  }
}
