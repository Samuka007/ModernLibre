use image::DynamicImage;
use pdf2image::{RenderOptionsBuilder, PDF as PDF2Image};

use crate::{
    models::{NewBook, NewBookBuilder},
    schema::books::extension,
};

pub fn get_metadata(
    pdf_buffer: Vec<u8>,
    filename: Option<&String>,
) -> Option<(NewBook, DynamicImage)> {
    let title = std::path::Path::new(filename?)
        .file_stem()?
        .to_str()?
        .to_string();
    let book = NewBookBuilder::default()
        .title(title)
        .extension("pdf".to_owned())
        .build()
        .unwrap();
    let cover = get_cover(pdf_buffer).unwrap();

    Some((book, cover))
}

fn get_cover(pdf_buffer: Vec<u8>) -> Option<DynamicImage> {
    let pdf_image = PDF2Image::from_bytes(pdf_buffer).ok()?;

    pdf_image
        .render(
            pdf2image::Pages::Single(0),
            RenderOptionsBuilder::default()
                .resolution(pdf2image::DPI::Uniform(150))
                .build()
                .ok()?,
        )
        .ok()?
        .into_iter()
        .next()
}
