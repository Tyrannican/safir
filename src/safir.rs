use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufWriter, Read, Result, Write},
    path::PathBuf,
};

#[derive(Default)]
pub struct Safir {
    pub store_loc: PathBuf,
    pub store: HashMap<String, String>,
}

impl Safir {
    pub fn init() -> Result<Self> {
        let mut s = Self::default();
        if let Some(home_dir) = dirs::home_dir() {
            let safir_path = home_dir.join(".safirstore");
            std::fs::create_dir_all(&safir_path)?;

            let safir_store_file = safir_path.join("safirstore.json");

            let mut f = OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(&safir_store_file)?;

            let mut contents = String::new();
            f.read_to_string(&mut contents)?;
            if contents.len() > 0 {
                let store = serde_json::from_str(&contents)?;
                s.store = store;
            } else {
                f.write_all(b"{}\n")?;
            }

            s.store_loc = safir_store_file.clone();
        }

        Ok(s)
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

    fn write(&self) -> Result<()> {
        let mut writer = BufWriter::new(File::create(&self.store_loc)?);
        serde_json::to_writer_pretty(&mut writer, &self.store)?;
        writer.write(b"\n")?;
        writer.flush()?;
        Ok(())
    }
}
