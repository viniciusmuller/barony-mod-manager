use std::collections::HashMap;
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
    pub is_active: bool,
    pub is_downloaded: bool, // Remove this field
    pub download_button: button::State,
    pub download_status: DownloadStatus,
}

#[derive(Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
pub struct SteamApiResponse<T> {
    pub response: T,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SteamWorkshopModResponse {
    // pub total: u8,
    #[serde(rename = "publishedfiledetails")]
    // From all my tests, it always contains an array with a single mod, so it's
    // probably safe to assume that it will always be like this :P
    pub mods: Vec<SteamWorkshopMod>,
}

/// TODO: Document this
#[derive(Deserialize, Debug, Clone)]
pub struct SteamWorkshopTotal {
    pub total: u64,
}

#[serde_with::serde_as]
#[derive(Deserialize, Debug, Clone)]
// TODO: Figure out about steam workshop dependencies and download mods' dependencies
pub struct SteamWorkshopMod {
    #[serde(rename = "publishedfileid")]
    pub id: String,
    pub title: String,
    pub file_size: String,
    pub preview_url: String,
    #[serde(rename = "file_description")]
    pub description: String,
    pub tags: Vec<SteamWorkshopTag>,
    pub vote_data: SteamWorkshopVoteData,
    pub views: u64,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub time_created: DateTime<Utc>,
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub time_updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct SteamWorkshopTag {
    pub tag: String,
    pub display_name: String,
}

impl PartialEq for SteamWorkshopTag {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}
impl Eq for SteamWorkshopTag {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SteamWorkshopVoteData {
    score: f64,
    pub votes_up: u64,
    pub votes_down: u64,
}
