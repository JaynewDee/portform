use std::fs::File;

use printpdf::*;

use super::consts;
use super::{api::DocumentData, file_io::ConfigFileHandler};

#[allow(dead_code)]
pub struct ResumeWriter {
    pub doc: (PdfDocumentReference, PdfPageIndex, PdfLayerIndex),
    pages: Option<Vec<PdfPage>>,
    fonts: Vec<IndirectFontRef>,
}

impl ResumeWriter {
    pub fn new(title: String) -> Self {
        Self {
            doc: PdfDocument::new(&title, Mm(consts::PAGE_X), Mm(consts::PAGE_Y), "L1"),
            pages: None,
            fonts: Vec::with_capacity(2),
        }
    }

    pub fn load_fonts(mut self) -> Self {
        let lucon = self
            .doc
            .0
            .add_external_font(File::open("assets/fonts/lucon.ttf").unwrap())
            .unwrap()
            .to_owned();
        self.fonts.push(lucon);
        self
    }

    pub fn get_primary_font(&self) -> IndirectFontRef {
        self.fonts[0].clone()
    }

    pub fn header_section(
        layer: PdfLayerReference,
        font: IndirectFontRef,
        name: String,
        profession: String,
    ) {
        layer.begin_text_section();

        let blue = color::Color::Rgb(Rgb::new(0.0, 0.0, 200.0, None));
        layer.set_font(&font, 24.0);
        layer.set_text_cursor(Mm(10.0), Mm(280.0));
        layer.set_line_height(20.0);
        layer.set_fill_color(blue);
        layer.set_word_spacing(300.0);
        layer.set_character_spacing(5.0);
        layer.set_text_rendering_mode(TextRenderingMode::Stroke);
        layer.write_text(name.clone(), &font);

        layer.set_font(&font, 14.0);
        layer.add_line_break();
        layer.set_line_offset(5.0);
        layer.write_text(profession.clone(), &font);
        layer.add_line_break();

        layer.end_text_section();
    }
}

pub fn assets(document: &super::api::DocumentShape) -> Result<DocumentData, anyhow::Error> {
    Ok(ConfigFileHandler::unpack(document)?)
}
