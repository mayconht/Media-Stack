use crate::{
    converter::{format::ConverterFormat, job::Job},
    http::response::ApiResponse,
    state::APP_STATE,
};
use actix_multipart::Multipart;
use actix_web::{post, HttpResponse, Responder, ResponseError};
use futures_util::StreamExt as _;
use log::info;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    #[error("no file uploaded")]
    NoFile,
    #[error("failed to get field")]
    GetField(#[from] actix_multipart::MultipartError),
    #[error("no filename provided")]
    NoFilename,
    #[error("missing file extension")]
    NoExtension,
    #[error("invalid file extension: {0}. allowed: jpg, png, gif")]
    InvalidExtension(String),
    #[error("failed to read file data")]
    GetChunk(#[from] actix_web::Error),
    #[error("internal server error while writing file")]
    WriteFile(#[from] std::io::Error),
    #[error("ffprobe failed to read file: {0}")]
    ParseFile(#[from] anyhow::Error),
}

impl ResponseError for UploadError {
    fn error_response(&self) -> HttpResponse {
        // change these status codes as needed
        let status = match self {
            UploadError::GetField(_) => actix_web::http::StatusCode::BAD_REQUEST,
            UploadError::GetChunk(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            UploadError::WriteFile(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            _ => actix_web::http::StatusCode::BAD_REQUEST,
        };

        HttpResponse::build(status).json(ApiResponse::<()>::Error(self.to_string()))
    }
}

#[post("/upload")]
pub async fn upload(mut payload: Multipart) -> Result<impl Responder, UploadError> {
    let mut job: Option<Job> = None;
    while let Some(item) = payload.next().await {
        let mut field = item?;

        if field.content_disposition().is_none() {
            continue;
        }

        let content_disposition = field.content_disposition().unwrap();
        if content_disposition.get_name() != Some("file") {
            continue;
        }

        // get file name
        let filename = content_disposition
            .get_filename()
            .ok_or_else(|| UploadError::NoFilename)?;

        let ext = filename
            .split('.')
            .last()
            .and_then(|ext| {
                Some(
                    ext.chars()
                        .filter(|c| c.is_alphanumeric())
                        .collect::<String>(),
                )
            })
            .ok_or_else(|| UploadError::NoExtension)?;

        if let Err(e) = ext.parse::<ConverterFormat>() {
            log::error!("failed to parse file extension: {}", e);
            return Err(UploadError::InvalidExtension(ext));
        }

        info!("uploaded file: {}", filename);

        let mut bytes = Vec::new();
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            bytes.extend_from_slice(&data);
        }
        let rand: [u8; 64] = rand::random();
        let token = hex::encode(rand);
        let our_job = Job::new(token, ext.to_string());
        job = Some(our_job.clone());
        let mut app_state = APP_STATE.lock().await;
        // fs::write(format!("input/{}.{}", our_job.id, ext), &bytes).await?;
        let mut file = File::create(format!("input/{}.{}", our_job.id, ext)).await?;
        file.write_all(&bytes).await?;
        file.flush().await?;
        drop(file);
        app_state.jobs.insert(our_job.id, our_job.clone());
        // spawn a new task which waits an hour before removing the job
        tokio::spawn(async move {
            tokio::time::sleep(crate::INPUT_LIFETIME).await;
            info!(
                "{:?} elapsed, removing {}",
                crate::INPUT_LIFETIME,
                our_job.id
            );
            let mut app_state = APP_STATE.lock().await;
            app_state.jobs.remove(&our_job.id);
            fs::remove_file(format!("input/{}.{}", our_job.id, ext))
                .await
                .ok();
        });
        break;
    }
    let mut job = job.ok_or_else(|| UploadError::NoFile)?;
    job.total_frames().await?;
    Ok(ApiResponse::Success(job))
}
