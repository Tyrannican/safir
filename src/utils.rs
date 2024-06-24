use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

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

/// Remove the .safirstore directory
pub fn purge_directory(path: impl AsRef<Path>) {
    if path.as_ref().exists() {
        std::fs::remove_dir_all(path).expect("unable to remove safirstore directory");
    }
}

/// Create the .safirstore directory in the user HOME
pub fn load_safir_workspace() -> PathBuf {
    match dirs::home_dir() {
        Some(home) => {
            let working_dir = home.join(".safirstore");
            fs::create_dir_all(&working_dir).expect("unable to create main directory");

            working_dir
        }
        None => {
            eprintln!("unable to obtain home directory path!");
            std::process::exit(-1);
        }
    }
}
