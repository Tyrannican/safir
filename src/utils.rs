// use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs;

pub fn check_rubin_installed() -> bool {
    if which::which("rubin").is_ok() {
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