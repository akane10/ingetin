use crate::{
    error::Error,
    event::{Event, Events},
    HasId,
};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::{
    fs::{self, File},
    io::BufReader,
};
use uuid::Uuid;

pub struct DB;

impl DB {
    fn get_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("db")
    }

    pub fn get(id: Uuid) -> Result<Event, Error> {
        let db_path = Self::get_path();
        let file_path = format!("{}/{}.json", db_path.to_str().unwrap(), id);
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);
        let data: Event = serde_json::from_reader(reader)?;

        Ok(data)
    }

    pub fn get_all() -> Result<Events, Error> {
        let mut data = Vec::new();
        let db_path = Self::get_path();

        for entry in fs::read_dir(db_path)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(file) = path.file_name() {
                let id: String = file
                    .to_str()
                    .unwrap()
                    .chars()
                    .take_while(|v| v != &'.')
                    .collect();

                match Uuid::parse_str(&id) {
                    Ok(uuid) => match Self::get(uuid) {
                        Ok(db) => data.push(db),
                        Err(e) => println!("err get {:#?}", e),
                    },
                    Err(e) => println!("err parse {:#?}", e),
                }
            }
        }

        Ok(data)
    }

    pub fn write<T: Serialize + HasId>(event: &T) -> Result<(), Error> {
        let db_path = Self::get_path();
        let file_path = format!("{}/{}.json", db_path.to_str().unwrap(), event.get_id());

        let file = File::create(file_path)?;
        serde_json::to_writer(file, &event)?;

        Ok(())
    }

    pub fn delete(id: Uuid) -> Result<(), Error> {
        let db_path = Self::get_path();
        let file_path = format!("{}/{}.json", db_path.to_str().unwrap(), id);
        fs::remove_file(file_path)?;

        Ok(())
    }

    pub fn delete_many(ids: &Vec<Uuid>) -> Result<(), Error> {
        for id in ids.iter() {
            match Self::delete(*id) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        }

        Ok(())
    }
}
