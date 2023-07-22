use super::os;
use crate::models::DocumentShape;
use serde::{Deserialize, Serialize};
use std::io::{BufWriter, Write};
pub struct ConfigFileHandler;

pub trait FileHandler<'a, T: Serialize + Deserialize<'a>> {
    fn read(filename: &str) -> Result<T, anyhow::Error>;
    fn write(data: T) -> Result<(), anyhow::Error>;
}

impl FileHandler<'_, DocumentShape> for ConfigFileHandler {
    fn read(_filename: &str) -> Result<DocumentShape, anyhow::Error> {
        unimplemented!();
        // Ok(())
    }

    fn write(data: DocumentShape) -> Result<(), anyhow::Error> {
        let json = serde_json::to_string(&data).unwrap();
        let temp_path = os::get_os_tempdir().0.join("portform_config.json");
        let temp_str = temp_path.as_os_str();
        println!("{:#?}", &temp_str);

        let file = std::fs::File::create(temp_str)?;
        let mut buf_writer = BufWriter::new(file);
        buf_writer.write_all(json.as_bytes())?;
        buf_writer.flush()?;
        Ok(())
    }
}
