use super::JobTrait;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use uuid::Uuid;

const DEFAULT_BITRATE: u64 = 4 * 1_000_000;
const BITRATE_MULTIPLIER: f64 = 2.5;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionJob {
    pub id: Uuid,
    pub auth: String,
    pub from: String,
    pub to: Option<String>,
    pub completed: bool,
    total_frames: Option<u64>,
    bitrate: Option<u64>,
}

impl ConversionJob {
    pub fn new(auth_token: String, from: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            auth: auth_token,
            from,
            to: None,
            completed: false,
            total_frames: None,
            bitrate: None,
        }
    }
    // TODO: scale based on resolution
    pub async fn bitrate(&mut self) -> anyhow::Result<u64> {
        // Ok(DEFAULT_BITRATE)
        if let Some(bitrate) = self.bitrate {
            return Ok(bitrate);
        }
        let output = Command::new("ffprobe")
            .args([
                "-v",
                "error",
                "-select_streams",
                "v:0",
                "-show_entries",
                "stream=bit_rate",
                "-of",
                "default=nokey=1:noprint_wrappers=1",
                &format!("input/{}.{}", self.id, self.from),
            ])
            .output()
            .await?;
        let bitrate = String::from_utf8(output.stdout)?;
        let bitrate = match bitrate.trim().parse::<u64>() {
            Ok(bitrate) => bitrate,
            Err(_) => DEFAULT_BITRATE,
        };
        self.bitrate = Some(bitrate);
        Ok(((bitrate as f64) * BITRATE_MULTIPLIER) as u64)
    }
    pub async fn total_frames(&mut self) -> anyhow::Result<u64> {
        if let Some(total_frames) = self.total_frames {
            return Ok(total_frames);
        }
        let output = Command::new("ffprobe")
            .args([
                "-v",
                "error",
                "-count_frames",
                "-select_streams",
                "v:0",
                "-show_entries",
                "stream=nb_read_frames",
                "-of",
                "default=nokey=1:noprint_wrappers=1",
                &format!("input/{}.{}", self.id, self.from),
            ])
            .output()
            .await?;
        let total_frames = String::from_utf8(output.stdout)?;
        let total_frames = total_frames.trim().parse::<u64>()?;
        self.total_frames = Some(total_frames);
        Ok(total_frames)
    }
}

impl JobTrait for ConversionJob {
    fn id(&self) -> Uuid {
        self.id
    }

    fn auth(&self) -> &str {
        &self.auth
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum ProgressUpdate {
    #[serde(rename = "frame", rename_all = "camelCase")]
    Frame(u64),
    #[serde(rename = "fps", rename_all = "camelCase")]
    FPS(f64),
}
