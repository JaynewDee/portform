use super::os;
use crate::models::DocumentShape;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
pub struct ConfigFileHandler;

pub trait FileHandler<'a, T: Serialize + Deserialize<'a>> {
    fn read() -> Result<T, anyhow::Error>;
    fn write(data: T) -> Result<(), anyhow::Error>;
}

impl FileHandler<'_, DocumentShape> for ConfigFileHandler {
    fn read() -> Result<DocumentShape, anyhow::Error> {
        let temp_path = os::get_os_config_path().0;
        let json = std::fs::read_to_string(temp_path)?;
        let document: DocumentShape = serde_json::from_str(&json)?;
        println!("{:#?}", document);
        Ok(document)
    }

    fn write(data: DocumentShape) -> Result<(), anyhow::Error> {
        let json = serde_json::to_string(&data).unwrap();
        let temp_path = os::get_os_config_path().0;

        let file = File::create(temp_path)?;
        let mut buf_writer = BufWriter::new(file);
        buf_writer.write_all(json.as_bytes())?;
        buf_writer.flush()?;
        Ok(())
    }
}
