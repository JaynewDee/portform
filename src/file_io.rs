use super::api::{DocumentData, DocumentShape};
use super::os;
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
        let temp_path = os::get_os_config_path()?.0;
        let json = std::fs::read_to_string(temp_path)?;
        let document_config: DocumentShape = serde_json::from_str(&json)?;

        Ok(document_config)
    }

    fn write(data: DocumentShape) -> Result<(), anyhow::Error> {
        let json = serde_json::to_string(&data).unwrap();
        let temp_path = os::get_os_config_path()?.0;

        let file = File::create(temp_path)?;
        let mut buf_writer = BufWriter::new(file);
        buf_writer.write_all(json.as_bytes())?;
        buf_writer.flush()?;
        Ok(())
    }
}

impl ConfigFileHandler {
    pub fn unpack(data: &DocumentShape) -> Result<DocumentData, anyhow::Error> {
        let unpacked = DocumentData {
            filename: data.filename.clone().unwrap(),
            title: data.title.clone().unwrap(),
            header: data.header.clone().unwrap(),
            summary: data.summary.clone().unwrap(),
            contact_details: data.contact_details.clone().unwrap(),
            employment_history: data.employment_history.clone().unwrap(),
            projects: data.projects.clone().unwrap(),
            skillset: data.skillset.clone().unwrap(),
            certifications: data.certifications.clone().unwrap(),
        };

        Ok(unpacked)
    }

    pub fn init_write_file(filename: String) -> BufWriter<File> {
        let write_file = std::fs::File::create(filename).unwrap();
        BufWriter::new(write_file)
    }
}
