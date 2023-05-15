//! Safir handles the logic of writing key/value pairs to disk
//!
//! Provides a simple wrapper around a Hash Map which stores values  with given keys
//! to a JSON file on disk so that you can load them up in other shell sessions.
//!
//! Prevents the faff around having to add rely on shell history or RC files for items
//! that you don't usually want to persist around.
//!
//! Safir gives you the option to add / get / remove items from the store
//! and to clear / purge when you're finished with them.
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufWriter, Read, Result, Write},
    path::PathBuf,
};

use colored::*;

/// Safir Store (fancy wrapper around reading and writing to a JSON file)
#[derive(Default)]
pub struct Safir {
    pub path: PathBuf,
    pub store: HashMap<String, String>,
}

impl Safir {
    /// Initialises the Safirstore if not already initialised
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

    /// Add an entry to the store and write it out to disk
    pub fn add_entry(&mut self, key: String, value: String) {
        self.store
            .entry(key)
            .and_modify(|entry| *entry = value.clone())
            .or_insert(value);

        self.write().expect("unable to write store out to file!");
    }

    /// Get an entry form the store by loading it from disk and displaying it
    pub fn get_entry(&self, key: String) {
        println!("{}", "--=Safirstore=--\n".bold());
        if let Some(val) = self.store.get(&key) {
            println!("{}: \"{}\"", key.bold().yellow(), val);
        } else {
            println!("{}: ", key.bold().yellow());
        }
    }

    /// Display all key/values in the store
    pub fn display_all(&self) {
        println!("{}", "--=Safirstore=--\n".bold());
        for (key, value) in self.store.iter() {
            println!("{}: \"{}\"\n", key.bold().yellow(), value);
        }
    }

    /// Remove a key/value pair from the store and update onto disk
    pub fn remove_entry(&mut self, key: String) {
        self.store.remove_entry(&key);
        self.write().expect("unable to update safirstore");
    }

    /// Outputs the key/value pair as a command with the prefix
    ///
    /// E.g. With a prefix of `alias` this will display the command as
    /// `alias {key}="{value}"` with {key} / {value} replaced with their values from the store
    pub fn set_commands(&self, prefix: &str, keys: &Vec<String>) {
        println!("{}", "--=Safirstore=--\n".bold());
        let prefix = match prefix {
            "alias" => "alias".bold().green(),
            "export" => "export".bold().magenta(),
            _ => prefix.bold(),
        };

        for key in keys {
            if let Some(value) = self.store.get(key) {
                println!("{} {}=\"{}\"\n", prefix, key.bold().yellow(), value);
            }
        }
    }

    /// Clear the the contents of the store and update onto disk
    pub fn clear_entries(&mut self) {
        if self.confirm_entry("Are you sure you want to clear the store?") {
            self.store.clear();
            self.write().expect("unable to clear safirstore");
        }
    }

    /// Remove the `.safirstore` directory and all contents
    pub fn purge(&mut self) {
        if self.confirm_entry("Are you sure you want to purge Safirstore?") {
            self.path.pop();
            std::fs::remove_dir_all(&self.path).expect("unable to remove safirstore directory");
        }
    }

    /// Confirmation dialog for important calls
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

    /// Load the contents of the store off of disk
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

    /// Write the contents of the store out to disk in the .safirstore/ directory
    fn write(&self) -> Result<()> {
        let mut writer = BufWriter::new(File::create(&self.path)?);
        serde_json::to_writer_pretty(&mut writer, &self.store)?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }
}
