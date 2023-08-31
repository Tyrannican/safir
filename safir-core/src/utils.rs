use anyhow::Result;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::config::SafirConfig;

use colored::*;
use sysinfo::{Pid, System, SystemExt};
use tokio::fs;

pub async fn init() -> Result<SafirConfig> {
    let store_dir = create_safir_directory().await?;
    let cfg = load_safir_config(&store_dir).await?;
    Ok(cfg)
}

pub fn check_rubin_installed() -> bool {
    if which::which("rubin").is_ok() {
        return true;
    }

    false
}

pub fn check_process_running(pid: u32) -> bool {
    let mut system = System::new_all();
    system.refresh_all();
    if system.process(Pid::from(pid as usize)).is_some() {
        return true;
    }

    false
}

pub fn is_safir_running(pid: Option<u32>) -> bool {
    match pid {
        Some(pid) => check_process_running(pid),
        None => false,
    }
}

pub async fn path_exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}

pub async fn create_safir_directory() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().unwrap();
    let store_path = home_dir.join(".safirstore");
    fs::create_dir_all(&store_path).await?;

    Ok(store_path)
}

#[cfg(target_family = "unix")]
pub async fn kill_process(pid: u32) -> Result<()> {
    if let Ok(process) = psutil::process::Process::new(pid) {
        if let Err(err) = process.kill() {
            eprintln!("failed to kill process: {}", err);
        }
    } else {
        eprintln!("failed to get process information");
    }

    Ok(())
}

#[cfg(target_os = "windows")]
pub async fn kill_process(pid: u32) {
    println!("*** Windows -- This is experimental and may not work as intended! ***");
    let output = Command::new("taskkill")
        .arg("/F")
        .arg("/PID")
        .arg(pid.to_string())
        .output()
        .expect("failed to call taskkill");

    if !output.status.success() {
        eprintln!("failed to terminate process");
    }
}

/// Formats and prints the message to stdout
pub fn print_output(msg: &str) {
    println!("{}\n", msg);
}

/// Prints the Safirstore header
pub fn print_header() {
    println!("{}", "--=Safirstore=--\n".bold());
}

/// Confirmation dialog for important calls
pub fn confirm_entry(msg: &str) -> bool {
    let mut answer = String::new();
    print!("{} (y/n) ", msg);
    std::io::stdout().flush().expect("failed to flush buffer");

    let _ = std::io::stdin()
        .read_line(&mut answer)
        .expect("unable to get input from user");

    let answer = answer.trim().to_lowercase();
    if answer == "y" {
        return true;
    }

    false
}

pub async fn load_safir_config(store_dir: impl AsRef<Path>) -> Result<SafirConfig> {
    let cfg_path = &store_dir.as_ref().join("safir.cfg");
    let mut cfg = if path_exists(&cfg_path).await {
        SafirConfig::load(&cfg_path).await?
    } else {
        SafirConfig::new()
    };

    cfg.root_path = store_dir.as_ref().to_owned();
    cfg.config_path = cfg_path.to_owned();

    // Used in cases where the process has ended ungracefully and the config hasnt been updated
    if let Some(pid) = cfg.memcache_pid {
        if !check_process_running(pid) {
            cfg = SafirConfig::new();
            cfg.write().await?;
        }
    }

    Ok(cfg)
}
