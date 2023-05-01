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

    pub fn get_entry(&self, key: String) -> String {
        if let Some(val) = self.store.get(&key) {
            return val.clone();
        }

        String::from("")
    }

    pub fn display_all(&self) {
        for (key, value) in self.store.iter() {
            println!("{}: {}", key, value);
        }
    }

    pub fn remove_entry(&mut self, key: String) {
        self.store.remove_entry(&key);
    }

    pub fn clear_entries(&mut self) {
        self.store.clear();
        self.write().expect("unable to clear safirstore");
    }

    fn load(&mut self) -> Result<()> {
        let mut f = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&self.path)?;

        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        if contents.len() > 0 {
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
        writer.write(b"\n")?;
        writer.flush()?;
        Ok(())
    }
}
