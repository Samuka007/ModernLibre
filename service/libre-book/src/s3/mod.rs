use file_format::FileFormat;
use image::DynamicImage;
use aws_sdk_s3::operation::{put_object, get_object};

mod presigned;
pub use presigned::*;
mod local;
pub use local::*;

/// Generate an S3 client configured with the environment variables
/// 
/// TODO:
/// - [ ] 支持热重载
pub fn s3_client() -> aws_sdk_s3::Client {
    let s3_endpoint = std::env::var("S3_ENDPOINT").expect("S3_ENDPOINT");
    let s3_region_var = std::env::var("S3_REGION").expect("S3_REGION");

    // AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY are used by the AWS SDK for Rust
    std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID");
    std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY");

    let runtime = actix_web::rt::Runtime::new().unwrap();
    let shared_config = runtime.block_on(
        aws_config::load_defaults(aws_config::BehaviorVersion::latest())
    );
    let s3_region = aws_sdk_s3::config::Region::new(s3_region_var);
    let conf = aws_sdk_s3::config::Builder::from(&shared_config)
        .endpoint_url(s3_endpoint)
        .region(s3_region)
        .build();

    aws_sdk_s3::Client::from_conf(conf)
}

pub struct StorageClient {
    client: aws_sdk_s3::Client,
    public_bucket_name: String,
    private_bucket_name: String,
}

impl StorageClient {
    pub fn new_from_env() -> Self {
        Self {
            client: s3_client(),
            public_bucket_name: std::env::var("PUBLIC_BUCKET").unwrap(),
            private_bucket_name: std::env::var("PRIVATE_BUCKET").unwrap()
        }
    }

    pub async fn upload_cover(
        &self,
        id: i32,
        cover: DynamicImage,
    ) -> Result<put_object::PutObjectOutput, put_object::PutObjectError> {
        let mut cursor = std::io::Cursor::new(Vec::new());
        // TODO: adjust the image size
        let _ = cover.write_to(&mut cursor, image::ImageFormat::Png);

        let key = format!("{}.png", id);
        let body = aws_sdk_s3::primitives::ByteStream::from(cursor.into_inner());

        self.client
            .put_object()
            .bucket(&self.public_bucket_name)
            .key(key)
            .body(body)
            .send()
            .await
            .map_err(|err| err.into_service_error())
    }

    pub async fn upload_book(
        &self,
        id: i32,
        file_format: FileFormat,
        body: Vec<u8>
    ) -> Result<put_object::PutObjectOutput, put_object::PutObjectError> {
        let key = format!("{}/{}.{}", id, id, file_format.extension());
        let body = aws_sdk_s3::primitives::ByteStream::from(body);

        self.client
            .put_object()
            .bucket(&self.private_bucket_name)
            .key(key)
            .body(body)
            .send()
            .await
            .map_err(|err| err.into_service_error())
    }

    pub async fn download_book_url(
        &self,
        id: i32,
        file_format: String,
    ) -> Result<String, get_object::GetObjectError> {
        let key = format!("{}/{}.{}", id, id, file_format);
        let expire_in = std::time::Duration::from_secs(60*60);
        // TODO: add expire_in to env

        get_presigned_download_url(
            &self.client,
            &key,
            &self.private_bucket_name,
            expire_in
        )
        .await
        .map(|presigned_request| presigned_request.uri().to_string())
    }
}
