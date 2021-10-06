use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;

pub struct BaronyMod {
    pub workshop_data: SteamWorkshopMod,
    pub is_active: bool,
    pub is_downloaded: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SteamApiResponse {
    response: SteamWorkshopModResponse
}

#[derive(Serialize, Deserialize)]
pub struct SteamWorkshopModResponse {
    total: u8,
    #[serde(rename = "publishedfiledetails")]
    // From all my tests, it always contains an array with a single mod, so it's
    // probably safe to assume that it will always be like this :P
    mods: Vec<SteamWorkshopMod>,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct SteamWorkshopMod {
    pub title: String,
    pub file_size: String,
    pub preview_url: String,
    #[serde(rename = "file_description")]
    pub description: String,
    pub tags: Vec<SteamWorkshopTag>,
    pub vote_data: SteamWorkshopVoteData,
    pub views: u64,
    #[serde_as(as = "TimestampMilliSeconds<String, Flexible>")]
    pub time_created: DateTime<Utc>,
    #[serde_as(as = "TimestampMilliSeconds<String, Flexible>")]
    pub time_updated: DateTime<Utc>,
    pub next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SteamWorkshopTag {
    tag: String,
    display_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SteamWorkshopVoteData {
    score: f64,
    votes_up: u64,
    votes_down: u64,
}
