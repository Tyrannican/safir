use std::fs;
use std::io;
use std::path::PathBuf;

pub fn create_safir_directory() -> io::Result<PathBuf> {
    let home_dir = dirs::home_dir().unwrap();
    let store_path = home_dir.join(".safirstore");
    fs::create_dir_all(&store_path)?;

    Ok(store_path)
}
