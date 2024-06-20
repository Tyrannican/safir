use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

/// Debug flag for testing without affecting my existing store
const DEBUG: bool = true;

/// Confirmation dialog for important calls
pub fn confirm_entry(msg: &str) -> bool {
    let mut answer = String::new();
    print!("{} (y/n) ", msg);
    std::io::stdout().flush().expect("failed to flush buffer");

    let _ = std::io::stdin()
        .read_line(&mut answer)
        .expect("unable to get input from user");

    let answer = answer.trim().to_lowercase();
    if answer == "y" || answer == "yes" {
        return true;
    }

    false
}

/// Outputs the Key-Value pair
pub fn display_kv(key: &str, value: &str) {
    println!("{key}=\"{value}\"")
}

/// Loads the store from disk
pub fn load_store(path: impl AsRef<Path>) -> HashMap<String, String> {
    let contents = std::fs::read_to_string(path.as_ref()).expect("unable to store contents");

    return serde_json::from_str::<HashMap<String, String>>(&contents)
        .expect("unable to deserialize store contents");
}

/// Writes the store to disk
pub fn write_store(store: &HashMap<String, String>, path: impl AsRef<Path>) {
    let str_store =
        serde_json::to_string_pretty(store).expect("unable to serialize store contents");

    let mut file = std::fs::File::create(&path).expect("unable to get file handle");

    file.write_all(str_store.as_bytes())
        .expect("unable to write store out to disk");
}

/// Remove the .safirstore directory
pub fn purge_directory(path: impl AsRef<Path>) {
    std::fs::remove_dir_all(path).expect("unable to remove safirstore directory");
}

/// Create the .safirstore directory in the user HOME
pub fn create_safir_workspace() -> PathBuf {
    let store_dir = if DEBUG {
        ".safirstore_debug"
    } else {
        ".safirstore"
    };

    if DEBUG {
        println!("DEBUG: Creating safir store at debug location");
    }

    match dirs::home_dir() {
        Some(home) => {
            let working_dir = home.join(store_dir);
            fs::create_dir_all(&working_dir).expect("unable to create main directory");

            working_dir
        }
        None => {
            eprintln!("unable to obtain home directory path!");
            std::process::exit(-1);
        }
    }
}
