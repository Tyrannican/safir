use std::io::{self, Write};
use std::path::PathBuf;

use colored::*;
use tokio::fs;

pub async fn create_safir_directory() -> io::Result<PathBuf> {
    let home_dir = dirs::home_dir().unwrap();
    let store_path = home_dir.join(".safirstore");
    fs::create_dir_all(&store_path).await?;

    Ok(store_path)
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
