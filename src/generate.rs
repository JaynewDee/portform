use chrono::prelude::*;
use printpdf::*;

#[allow(dead_code)]
pub struct ResumeGenerator {
    title: String,
    pub filename: String,
    pub doc: (PdfDocumentReference, PdfPageIndex, PdfLayerIndex),
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
