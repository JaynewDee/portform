use chrono::prelude::*;
use printpdf::*;

#[allow(dead_code)]
pub struct ResumeWriter {
    pub doc: (PdfDocumentReference, PdfPageIndex, PdfLayerIndex),
    pages: Option<Vec<PdfPage>>,
}

impl ResumeWriter {
    pub fn new() -> Self {
        let date_string = FormattedDate::from(Utc::now());

        let title = "";

        Self {
            doc: PdfDocument::new(title, Mm(210.0), Mm(297.0), "L1"),
            pages: None,
        }
    }
}

struct FormattedDate(String);

impl From<DateTime<Utc>> for FormattedDate {
    fn from(value: DateTime<Utc>) -> Self {
        let from_utc_now = value
            .to_string()
            .chars()
            .map(|c| match c {
                ':' | ' ' => '-',
                _ => c,
            })
            .collect();

        Self(from_utc_now)
    }
}
