use actix_multipart::form::MultipartForm;
use actix_web::{web, HttpResponse};
use aws_sdk_s3::Client;
use log::info;
use std::io::Read;

use crate::s3;

// pub async fn oss_temp_credential(
//     book_id: web::Path<u32>,
//     client: web::Data<s3::StorageClient>
// ) -> HttpResponse {
//     let url = client.download_book_url(book_id, file_format)
// }