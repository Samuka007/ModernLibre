use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use std::path::Path;

// 临时文件流
#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub file: TempFile,
}

#[derive(Debug)]
pub enum FileType {
    Epub,
    Pdf,
    Unknown,
}

impl FileType {
    pub fn from_filename(filename: &str) -> Self {
        Path::new(filename)
            .extension()
            .and_then(|os_str| os_str.to_str())
            .map_or(Self::Unknown, FileType::from_str)
    }

    #[allow(unused)]
    pub fn from_str(ftype: &str) -> Self {
        match ftype.to_lowercase().as_str() {
            "epub" => Self::Epub,
            "pdf" => Self::Pdf,
            _ => Self::Unknown,
        }
    }

    #[allow(unused)]
    pub fn as_str(&self) -> &str {
        match self {
            FileType::Epub => "epub",
            FileType::Pdf => "pdf",
            FileType::Unknown => "unknown",
        }
    }
}
