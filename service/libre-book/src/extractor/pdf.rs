use image::DynamicImage;
use pdf2image::{RenderOptionsBuilder, PDF as PDF2Image};

use crate::models::NewBook;

pub fn get_metadata(pdf_buffer: Vec<u8>) -> Option<(NewBook, DynamicImage)> {
    let pdf_image = PDF2Image::from_bytes(pdf_buffer).ok()?;
    
    let cover = || -> Option<DynamicImage> {
        pdf_image.render(
            pdf2image::Pages::Single(0),
            RenderOptionsBuilder::default()
                .resolution(pdf2image::DPI::Uniform(150))
                .build()
                .ok()?,
        ).ok()?.into_iter().next()
    }().unwrap_or_default();

    ( , cover)
    todo!()
}
