use actix_web::{web, HttpResponse};
use crate::s3::StorageClient;

#[derive(serde::Deserialize)]
pub struct DownloadQuery {
    id: i32,
    ext: String,
}

pub async fn oss_temp_credential(
    query: web::Query<DownloadQuery>, client: web::Data<StorageClient>
) -> Result<HttpResponse, actix_web::Error> {
    let DownloadQuery { id, ext } = query.into_inner();

    client.download_book_url(id, ext).await
        .map(|url| 
            HttpResponse::Found()
                .append_header(("Location", url))
                .finish()
        )
        .map_err(|err| actix_web::error::ErrorNotFound(err))
}
