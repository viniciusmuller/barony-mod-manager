use std::fmt::{self, Display};

use chrono::{DateTime, Utc};
use iced::button;
use iced::image::Handle;
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

#[derive(Debug, Clone)]
pub struct BaronyMod {
    pub workshop: SteamWorkshopMod,
    pub image_handle: Handle,
    pub download_button: button::State,
    pub download_status: DownloadStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadStatus {
    Downloaded,
    NotDownloaded,
    Preparing,
    Downloading,
    ErrorOccurred,
}

impl Display for DownloadStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DownloadStatus::Downloaded => "Downloaded",
                DownloadStatus::NotDownloaded => "Not downloaded",
                DownloadStatus::Preparing => "Preparing download...",
                DownloadStatus::Downloading => "Downloading...",
                DownloadStatus::ErrorOccurred => "Error occurred. Could not download the mod.",
            }
        )
    }
}

#[serde_with::serde_as]
#[derive(Deserialize, Debug, Clone)]
// TODO: Figure out about steam workshop dependencies and download mods' dependencies
pub struct SteamWorkshopMod {
    pub id: String,
    pub title: String,
    pub file_size: u64,
    pub preview_url: String,
    pub description: String,
    pub tags: Vec<String>,
    pub votes: SteamWorkshopVoteData,
    pub views: u64,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub time_created: DateTime<Utc>,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub time_updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SteamWorkshopVoteData {
    pub up: u64,
    pub down: u64,
}
