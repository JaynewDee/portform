use chrono::prelude::*;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub struct PDFGenerator {
    title: String,
    filename: String,
    doc: (PdfDocumentReference, PdfPageIndex, PdfLayerIndex),
    pages: Option<Vec<PdfPage>>,
}

impl Default for PDFGenerator {
    fn default() -> Self {
        let title = "My_PDF".to_string();
        let date_string = Utc::now().to_string().replace(" ", "_");
        let filename = format!("{}.pdf", date_string);

        println!("{filename}");
        Self {
            title: title.clone(),
            filename,
            doc: PdfDocument::new(&title, Mm(247.0), Mm(210.0), "L1"),
            pages: None,
        }
    }
}

impl PDFGenerator {
    pub fn initialize() {}
}

fn main() {
    let generator = PDFGenerator::default();
    let (doc, pgIdx, layerIdx) = generator.doc;

    let file = File::create(generator.filename).unwrap();
    let buffer_writer = &mut BufWriter::new(file);
    doc.save(buffer_writer).unwrap();
}
