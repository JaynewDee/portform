use std::fs::File;

use printpdf::*;

use super::consts;

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

        let _blue = color::Color::Rgb(Rgb::new(0.0, 0.0, 200.0, None));
        layer.set_font(&font, 24.0);
        layer.set_text_cursor(Mm(10.0), Mm(285.0));
        layer.set_line_height(20.0);
        layer.set_character_spacing(3.0);
        layer.set_text_rendering_mode(TextRenderingMode::Stroke);
        layer.write_text(name.clone(), &font);

        layer.set_font(&font, 13.0);
        layer.add_line_break();
        layer.set_line_offset(5.0);
        layer.set_text_rendering_mode(TextRenderingMode::Stroke);
        layer.write_text(profession.clone(), &font);
        layer.add_line_break();

        layer.end_text_section();
    }

    pub fn contact_section(
        layer: PdfLayerReference,
        font: IndirectFontRef,
        data: (String, String, String, String),
    ) {
        let (email, website, phone, address) = data;

        layer.begin_text_section();

        layer.set_line_height(10.0);
        layer.set_character_spacing(1.0);
        layer.set_text_rendering_mode(TextRenderingMode::Stroke);

        layer.set_font(&font, 10.0);
        layer.set_text_cursor(Mm(10.0), Mm(270.0));
        layer.write_text("CONTACT DETAILS", &font);

        layer.set_line_height(11.0);

        let underline = Line {
            points: vec![(Point::new(Mm(160.0), Mm(275.0)), true)],
            is_closed: true,
            has_fill: true,
            has_stroke: true,
            is_clipping_path: false,
        };

        layer.set_fill_color(color::Color::Rgb(Rgb::new(0.0, 0.0, 210.0, None)));
        layer.add_shape(underline);

        layer.add_line_break();
        layer.set_font(&font, 9.0);
        layer.write_text(format!("EMAIL:      {}", &email), &font);

        layer.add_line_break();
        layer.write_text(format!("PORTFOLIO:  {}", &website), &font);

        layer.add_line_break();
        layer.write_text(format!("PHONE #:    {}", phone), &font);

        layer.add_line_break();
        layer.write_text(format!("LOCATION:   {}", address), &font);

        layer.end_text_section();
    }
}

#[allow(dead_code, unused_variables)]
pub fn assets(document: &super::api::DocumentShape) -> Result<(), anyhow::Error> {
    Ok(())
}
