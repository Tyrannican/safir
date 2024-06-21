use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

/// Debug flag for testing without affecting my existing store
const DEBUG: bool = true;

/// Type to represent a KVPair
pub type KVPair = (String, String);

/// Confirmation dialog for important calls
pub fn confirm_entry(msg: &str) -> bool {
    let mut answer = String::new();
    print!("{} (y/n) ", msg);
    std::io::stdout().flush().expect("failed to flush buffer");

    let _ = std::io::stdin()
        .read_line(&mut answer)
        .expect("unable to get input from user");

    let answer = answer.trim().to_lowercase();

    answer == "y" || answer == "yes"
}

/// Outputs multiple KV pairs
pub fn display_multiple_kv(kvs: Vec<KVPair>) {
    for kv in kvs.into_iter() {
        let (key, value) = kv;
        println!("{key}=\"{value}\"");
    }
}

/// Output key-value pairs with a leading string (e.g. alias or export)
pub fn custom_display(display_cmd: &str, kvs: Vec<KVPair>) {
    for kv in kvs.into_iter() {
        let (key, value) = kv;
        println!("{display_cmd} {key}=\"{value}\"");
    }
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
    if path.as_ref().exists() {
        std::fs::remove_dir_all(path).expect("unable to remove safirstore directory");
    }
}

/// Create the .safirstore directory in the user HOME
pub fn load_safir_workspace() -> PathBuf {
    let store_dir = if DEBUG {
        ".debug_safirstore"
    } else {
        ".safirstore"
    };

    match dirs::home_dir() {
        Some(home) => {
            let working_dir = home.join(store_dir);
            if DEBUG && !working_dir.exists() {
                println!("DEBUG: Creating safir store at debug location");
            }
            fs::create_dir_all(&working_dir).expect("unable to create main directory");

            working_dir
        }
        None => {
            eprintln!("unable to obtain home directory path!");
            std::process::exit(-1);
        }
    }
}
