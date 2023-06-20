// use std::fs;
use std::io;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn create_safir_directory() -> io::Result<PathBuf> {
    let home_dir = dirs::home_dir().unwrap();
    let store_path = home_dir.join(".safirstore");
    fs::create_dir_all(&store_path).await?;

    Ok(store_path)
}

pub async fn write_pid_file(store_dir: &PathBuf, pid: u32) -> io::Result<()> {
    let fp = store_dir.join("rubin_server.pid");
    let mut pid_file = fs::File::create(fp).await?;
    pid_file.write_all(format!("{}", pid).as_bytes()).await?;

    Ok(())
}

pub async fn load_pid(store_dir: &PathBuf) -> io::Result<u32> {
    let fp = store_dir.join("rubin_server.pid");
    let mut pid_file = fs::File::open(fp).await?;
    let mut buf = vec![];

    pid_file.read_to_end(&mut buf).await?;
    let pid = String::from_utf8_lossy(&buf).parse::<u32>().unwrap();

    Ok(pid)
}

pub async fn remove_pid_file(store_dir: &PathBuf) -> io::Result<()> {
    let fp = store_dir.join("rubin_server.pid");
    fs::remove_file(fp).await?;

    Ok(())
}

#[cfg(target_family = "unix")]
pub async fn kill_process(store_dir: &PathBuf, pid: u32) -> io::Result<()> {
    if let Ok(process) = psutil::process::Process::new(pid) {
        if let Err(err) = process.kill() {
            eprintln!("failed to kill process: {}", err);
        } else {
            remove_pid_file(store_dir).await?;
        }
    } else {
        eprintln!("failed to get process information");
    }

    Ok(())
}

#[cfg(target_os = "windows")]
pub async fn kill_process(store_dir: &PathBuf, pid: u32) {
    println!("*** Windows -- This is experimental and may not work as intended! ***");
    let output = Command::new("taskkill")
        .arg("/F")
        .arg("/PID")
        .arg(pid.to_string())
        .output()
        .expect("failed to call taskkill");

    if !output.status.success() {
        eprintln!("failed to terminate process");
    } else {
        remove_pid_file(store_dir).await?;
    }
}
