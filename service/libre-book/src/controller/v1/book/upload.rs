use actix_multipart::form::{tempfile::TempFile, MultipartForm};

use actix_web::{web, HttpResponse};
use aws_sdk_s3::Client as S3Client;
use std::io::Read;
use file_format::FileFormat;

use crate::{s3, extractor};

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
        FileFormat::PortableDocumentFormat => {
            todo!("pdf")
        }
        FileFormat::ElectronicPublication => {
            extractor::epub::get_metadata(buffer)
            .ok_or(ErrorBadRequest("Invalid epub file"))?
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
