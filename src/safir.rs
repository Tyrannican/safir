use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufWriter, Read, Result, Write},
    path::PathBuf,
};

#[derive(Default)]
pub struct Safir {
    pub path: PathBuf,
    pub store: HashMap<String, String>,
}

impl Safir {
    pub fn init() -> Result<Self> {
        let mut safir = Self::default();
        if let Some(home_dir) = dirs::home_dir() {
            let safir_path = home_dir.join(".safirstore");
            std::fs::create_dir_all(&safir_path)?;
            safir.path = safir_path.join("safirstore.json");
            safir.load()?;
        }

        Ok(safir)
    }

    pub fn add_entry(&mut self, key: String, value: String) {
        self.store
            .entry(key)
            .and_modify(|entry| *entry = value.clone())
            .or_insert(value);

        self.write().expect("unable to write store out to file!");
    }

    pub fn get_entry(&self, key: String) {
        println!("--=Safirstore=--\n");
        if let Some(val) = self.store.get(&key) {
            println!("{}: \"{}\"", key, val);
        } else {
            println!("{}: ", key);
        }
        println!();
    }

    pub fn display_all(&self) {
        println!("--=Safirstore=--\n");
        for (key, value) in self.store.iter() {
            println!("{}: \"{}\"", key, value);
        }
        println!();
    }

    pub fn remove_entry(&mut self, key: String) {
        self.store.remove_entry(&key);
        self.write().expect("unable to update safirstore");
    }

    pub fn set_commands(&self, prefix: &str, keys: &Vec<String>) {
        println!("--=Safirstore=--\n");
        for key in keys {
            if let Some(value) = self.store.get(key) {
                println!("{} {}=\"{}\"", prefix, key, value);
            }
        }
        println!();
    }

    pub fn clear_entries(&mut self) {
        if self.confirm_entry("Are you sure you want to clear the store?") {
            self.store.clear();
            self.write().expect("unable to clear safirstore");
        }
    }

    pub fn purge(&mut self) {
        if self.confirm_entry("Are you sure you want to purge Safirstore?") {
            self.path.pop();
            std::fs::remove_dir_all(&self.path).expect("unable to remove safirstore directory");
        }
    }

    fn confirm_entry(&self, msg: &str) -> bool {
        let mut answer = String::new();
        print!("{} (y/n) ", msg);
        std::io::stdout().flush().expect("failed to flush buffer");

        let _ = std::io::stdin()
            .read_line(&mut answer)
            .expect("unable to get input from user");

        let answer = answer.trim();
        if answer == "y" || answer == "Y" {
            return true;
        }

        false
    }

    fn load(&mut self) -> Result<()> {
        let mut f = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&self.path)?;

        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        if !contents.is_empty() {
            let store = serde_json::from_str(&contents)?;
            self.store = store;
        } else {
            self.write()?;
        }

        Ok(())
    }

    fn write(&self) -> Result<()> {
        let mut writer = BufWriter::new(File::create(&self.path)?);
        serde_json::to_writer_pretty(&mut writer, &self.store)?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }
}
