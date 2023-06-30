use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::cfg::SafirConfig;

use colored::*;
use sysinfo::{Pid, System, SystemExt};
use tokio::fs;

pub fn check_rubin_installed() -> bool {
    if which::which("rubin").is_ok() {
        return true;
    }

    false
}

pub fn check_process_running(pid: u32) -> bool {
    let mut system = System::new_all();
    system.refresh_all();
    if let Some(_) = system.process(Pid::from(pid as usize)) {
        return true;
    }

    false
}

pub async fn path_exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}

pub async fn create_safir_directory() -> io::Result<PathBuf> {
    let home_dir = dirs::home_dir().unwrap();
    let store_path = home_dir.join(".safirstore");
    fs::create_dir_all(&store_path).await?;

    Ok(store_path)
}

#[cfg(target_family = "unix")]
pub async fn kill_process(pid: u32) -> io::Result<()> {
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
    println!("{}", format!("{}\n", msg));
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

pub async fn load_safir_config(safir_cfg: impl AsRef<Path>) -> io::Result<SafirConfig> {
    let mut cfg = if path_exists(&safir_cfg).await {
        SafirConfig::load(&safir_cfg).await?
    } else {
        SafirConfig::new()
    };

    // Used in cases where the process has ended ungracefully and the config hasnt been updated
    match cfg.memcache_pid {
        Some(pid) => {
            if !check_process_running(pid) {
                cfg = SafirConfig::new();
                cfg.write(&safir_cfg).await?;
            }
        }
        None => {}
    }

    Ok(cfg)
}
