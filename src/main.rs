use chrono::prelude::*;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

mod cli;
mod errors;
mod models;

use errors::Error;

fn main() -> Result<(), Error> {
    // Get input commands and their data

    cli::CLParser::handle_arguments();

    let generator = ResumeGenerator::new("My_New_Resume".to_string());
    let (doc, _pg_idx, _layer_idx) = generator.doc;

    let file = File::create(generator.filename)?;
    let buffer_writer = &mut BufWriter::new(file);

    doc.save(buffer_writer)?;

    Ok(())
}

#[allow(dead_code)]
pub struct ResumeGenerator {
    title: String,
    filename: String,
    doc: (PdfDocumentReference, PdfPageIndex, PdfLayerIndex),
    pages: Option<Vec<PdfPage>>,
}

impl ResumeGenerator {
    pub fn new(title: String) -> Self {
        let date_string: String = Utc::now()
            .to_string()
            .chars()
            .map(|c| match c {
                ':' | ' ' => '-',
                _ => c,
            })
            .collect();

        let filename = format!("{}.pdf", date_string);

        Self {
            title: title.clone(),
            filename,
            doc: PdfDocument::new(&title, Mm(210.0), Mm(297.0), "L1"),
            pages: None,
        }
    }
}
