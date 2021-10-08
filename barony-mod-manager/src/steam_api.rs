// use iced::futures::executor::block_on;

// use std::future::Future;

use std::time::Duration;

use iced::{button, image::Handle};
use image::io::Reader;
use reqwest::{Client, Error};
use serde_json::json;

use crate::{
    data::{
        BaronyMod, DownloadStatus, SteamApiResponse, SteamWorkshopModResponse, SteamWorkshopTotal,
    },
    images::{resize, to_handle},
};

static BARONY_APP_ID: u64 = 371970;
static APP_IMAGES_SIZE: u32 = 180; // Pixels

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
) -> Result<BaronyMod, String> {
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

    let steam_mod = mod_.response.mods.pop().unwrap();

    // TODO: Use include_bytes! here
    let image = Reader::open("resources/img/no_image.png")
        .unwrap()
        .decode()
        .unwrap();

    let resized = resize(&image, APP_IMAGES_SIZE, APP_IMAGES_SIZE);
    let default_handle = to_handle(&resized);

    let image_handle = if steam_mod.preview_url.is_empty() {
        default_handle
    } else {
        match download_image(client, steam_mod.preview_url.clone()).await {
            Ok(handle) => handle,
            Err(_err) => default_handle,
        }
    };

    let barony_mod = BaronyMod {
        // TODO: To check if mod is download when building, one will have to have the barony's path
        // in hands. Probably break this into two functions: download_from_workshop and
        // build_barony_mod which are called from the update function
        is_downloaded: false, //is_mod_downloaded(steam_mod.title.clone()),
        workshop: steam_mod.clone(),
        image_handle,
        download_button: button::State::new(),
        download_status: DownloadStatus::NotDownloaded,
    };

    Ok(barony_mod)
}

pub async fn download_image(client: Client, url: String) -> Result<Handle, Error> {
    let image_bytes = client.get(url).send().await?.bytes().await?;
    let image = image::load_from_memory(&image_bytes).unwrap();
    let resized = resize(&image, APP_IMAGES_SIZE, APP_IMAGES_SIZE);
    Ok(to_handle(&resized))
}
