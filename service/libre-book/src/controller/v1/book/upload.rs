use actix_multipart::form::{tempfile::TempFile, MultipartForm};

use actix_web::{web, HttpResponse};
use aws_sdk_s3::Client as S3Client;
use file_format::FileFormat;
use std::io::Read;

use crate::{extractor, models::NewBookBuilder, s3, util::parse_file_name};

// 临时文件流
#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub file: TempFile,
}

pub async fn upload(
    mut payload: MultipartForm<UploadForm>,
    storage: web::Data<S3Client>,
) -> Result<HttpResponse, actix_web::Error> {
    use actix_web::error::*;

    let mut buffer = Vec::new();
    payload.file.file.read_to_end(&mut buffer)?;
    let buffer = buffer;

    let (book, cover) = match FileFormat::from_bytes(&buffer) {
        FileFormat::PortableDocumentFormat => (
            NewBookBuilder::with_defaults()
                .title(parse_file_name(payload.file.file_name.as_ref().unwrap()))
                .build()
                .map_err(|_| ErrorBadRequest("Invalid pdf file"))?,
            extractor::pdf::get_metadata(buffer).ok_or(ErrorBadRequest("Invalid pdf file"))?,
        ),
        FileFormat::ElectronicPublication => {
            extractor::epub::get_metadata(buffer).ok_or(ErrorBadRequest("Invalid epub file"))?
        }
        FileFormat::PlainText => {
            todo!("txt")
        }
        _ => {
            return Err(ErrorBadRequest("Unknown file format"));
        }
    };

    todo!("Extract epub file");
}
