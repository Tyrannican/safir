use std::{io::Write, collections::HashMap, path::Path};

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

pub fn display_kv(key: &str, value: &str) {
    println!("{key}=\"{value}\"")
}

pub fn load_store(path: impl AsRef<Path>) -> HashMap<String, String> {
    let contents = std::fs::read_to_string(path.as_ref())
        .expect("unable to store contents");

    return serde_json::from_str::<HashMap<String, String>>(&contents)
        .expect("unable to deserialize store contents");
}

pub fn write_store(store: &HashMap<String, String>, path: impl AsRef<Path>) {
    let str_store = serde_json::to_string_pretty(store)
        .expect("unable to serialize store contents");

    let mut file = std::fs::File::create(&path)
        .expect("unable to get file handle");

    file.write_all(str_store.as_bytes())
        .expect("unable to write store out to disk");
}

pub fn purge_directory(path: impl AsRef<Path>) {
    std::fs::remove_dir_all(path)
        .expect("unable to remove safirstore directory");
}
