use actix_multipart::form::{tempfile::TempFile, MultipartForm};

use actix_web::{web, HttpResponse};
use diesel::insert_into;
use diesel_async::RunQueryDsl;
use file_format::FileFormat;
use std::io::Read;

use crate::{extractor, s3, schema};

// 临时文件流
#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub file: TempFile,
}

pub async fn upload(
    mut payload: MultipartForm<UploadForm>,
    storage: web::Data<s3::StorageClient>,
    postgres: web::Data<libre_core::database::postgres::PostgresPool>,
) -> Result<HttpResponse, actix_web::Error> {
    use actix_web::error::*;

    let mut buffer = Vec::new();
    payload.file.file.read_to_end(&mut buffer)?;
    let body = buffer.clone();

    let file_format = FileFormat::from_bytes(&buffer);

    let (book, cover) = match file_format {
        FileFormat::PortableDocumentFormat => {
            extractor::pdf::get_metadata(buffer, payload.file.file_name.as_ref())
                .ok_or(ErrorBadRequest("Invalid pdf file"))?
        },
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

    let mut pg_conn = postgres.get().await?;

    use schema::books::dsl;
    let id: i32 = insert_into(dsl::books)
        .values(&book)
        .returning(dsl::id)
        .get_result(&mut pg_conn)
        .await
        .map_err(|_| ErrorInternalServerError("Failed to insert book"))?;

    storage.upload_book(id, file_format, body).await
        .map_err(|err| ErrorInternalServerError(err.to_string()))?;
    storage.upload_cover(id, cover).await
        .map_err(|err| ErrorInternalServerError(err.to_string()))?;

    Ok(HttpResponse::Created().finish())
}
