use actix_web::{web, HttpResponse};
use aws_sdk_s3::Client;

use crate::s3;

pub async fn download(bookid: web::Path<i32>, client: web::Data<Client>) -> HttpResponse {
    // info!("download book: {}", bookid);
    let mut file_stream = match s3::download(&client, &bookid.to_string(), "librebooks").await {
        Ok(output) => output.body,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to download file!"),
    };

    let mut temp_stream = Vec::new();
    while let Ok(Some(bytes)) = file_stream
        .try_next()
        .await
        .map_err(|err| HttpResponse::InternalServerError().json(err.to_string()))
    {
        temp_stream.extend_from_slice(&bytes);
    }

    HttpResponse::Ok()
        .content_type("application/epub+zip")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}.epub\"", bookid),
        ))
        .body(temp_stream)
}

pub async fn oss_temp_credential(client: web::Data<Client>) -> HttpResponse {
    HttpResponse::Ok().json(
        s3::get_presigned_download_url(
            &client,
            "test",                             // TODO
            "test",                             // TODO
            std::time::Duration::from_secs(60), // TODO
        )
        .await
        .unwrap()
        // PresignedRequest doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/presigning/struct.PresignedRequest.html
        // Error doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/operation/get_object/enum.GetObjectError.html
        // TODO
        // - [ ] handle error
        .uri(),
    )
}
