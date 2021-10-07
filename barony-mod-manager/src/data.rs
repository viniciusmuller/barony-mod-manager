use chrono::{DateTime, Utc};
use image::{DynamicImage, ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

#[derive(Debug, Clone)]
pub struct BaronyMod {
    pub workshop: SteamWorkshopMod,
    pub image: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    pub is_active: bool,
    pub is_downloaded: bool,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SteamWorkshopVoteData {
    score: f64,
    votes_up: u64,
    votes_down: u64,
}
