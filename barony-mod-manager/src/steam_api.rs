// use iced::futures::executor::block_on;

// use std::future::Future;

use std::time::Duration;

use iced::{button, image::Handle};
use reqwest::{Client, Error};
use serde_json::json;

use crate::{
    data::{
        BaronyMod, DownloadStatus, SteamApiResponse, SteamWorkshopMod, SteamWorkshopModResponse,
        SteamWorkshopTotal,
    },
    filesystem::is_mod_downloaded,
    images::{resize, to_handle},
};

static BARONY_APP_ID: u64 = 371970;
static APP_IMAGES_SIZE: u32 = 180; // Pixels
static DEFAULT_IMAGE: &'static [u8; 4921] = include_bytes!("../resources/img/no_image.png");

pub async fn get_total_mods(client: Client, steam_key: String) -> Result<u64, String> {
    let params = json!({
        "key": steam_key,
        "appid": BARONY_APP_ID.to_string()
    });

    let response = client
        .get("https://api.steampowered.com/IPublishedFileService/QueryFiles/v1")
        .query(&params)
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() != 403 {
                let json = response
                    .json::<SteamApiResponse<SteamWorkshopTotal>>()
                    .await
                    .unwrap();

                Ok(json.response.total)
            } else {
                Err("Not authenticated. Invalid Steam API key.".to_string())
            }
        }
        Err(_message) => {
            Err("Could not connect to Steam. Is your internet connection up?".to_string())
        }
    }
}

pub async fn get_workshop_item(
    client: Client,
    steam_key: String,
    mod_number: u64,
) -> Result<SteamWorkshopMod, String> {
    let params = json!({
        "key": steam_key,
        "appid": BARONY_APP_ID,
        "return_tags": true,
        "return_vote_data": true,
        "strip_description_bbcode": true,
        "page": mod_number
    });

    // Some sleep so that steam doesn't rate limit us for slapping 100+ concurrent
    // requests on them.
    let duration = Duration::from_millis(mod_number * 30);
    async_std::task::sleep(duration).await;

    let response = client
        .get("https://api.steampowered.com/IPublishedFileService/QueryFiles/v1")
        .query(&params)
        .send()
        .await;

    let mut mod_ = match response {
        Ok(response) => response
            .json::<SteamApiResponse<SteamWorkshopModResponse>>()
            .await
            .unwrap(),
        Err(_error) => return Err("Failed to get mod data.".to_string()),
    };

    let workshop_mod = mod_.response.mods.pop().unwrap();
    Ok(workshop_mod)
}

pub async fn build_barony_mod(
    client: Client,
    barony_dir: String,
    workshop_data: SteamWorkshopMod,
) -> BaronyMod {
    let image = image::load_from_memory(DEFAULT_IMAGE).unwrap();
    let resized = resize(&image, APP_IMAGES_SIZE, APP_IMAGES_SIZE);
    let default_handle = to_handle(&resized);

    let image_handle = if workshop_data.preview_url.is_empty() {
        default_handle
    } else {
        match download_image(client, workshop_data.preview_url.clone()).await {
            Ok(handle) => handle,
            Err(_err) => default_handle,
        }
    };

    BaronyMod {
        is_downloaded: is_mod_downloaded(&barony_dir, &workshop_data.title.clone()),
        workshop: workshop_data,
        image_handle,
        download_button: button::State::new(),
        download_status: DownloadStatus::NotDownloaded,
    }
}

pub async fn download_image(client: Client, url: String) -> Result<Handle, Error> {
    let image_bytes = client.get(url).send().await?.bytes().await?;
    let image = image::load_from_memory(&image_bytes).unwrap();
    let resized = resize(&image, APP_IMAGES_SIZE, APP_IMAGES_SIZE);
    Ok(to_handle(&resized))
}
