use iced::{button, image::Handle};
use reqwest::{Client, Error};

use crate::{
    data::{BaronyMod, DownloadStatus, SteamWorkshopMod},
    filesystem::is_mod_downloaded,
    images::{resize, to_handle},
};

static APP_IMAGES_SIZE: u32 = 180; // Pixels
static DEFAULT_IMAGE: &[u8; 4921] = include_bytes!("../resources/img/no_image.png");

pub async fn get_barony_workshop_mods(
    client: Client,
) -> Result<Vec<SteamWorkshopMod>, reqwest::Error> {
    let endpoint =
        "https://raw.githubusercontent.com/arcticlimer/barony-mod-manager/master/data/mods.json";
    let response = client
        .get(endpoint)
        .send()
        .await?
        .json::<Vec<SteamWorkshopMod>>()
        .await?;

    Ok(response)
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
        // is_downloaded: is_mod_downloaded(&barony_dir, &workshop_data.title.clone()),
        image_handle,
        download_button: button::State::new(),
        download_status: if is_mod_downloaded(&barony_dir, &workshop_data.title.clone()) {
            DownloadStatus::Downloaded
        } else {
            DownloadStatus::NotDownloaded
        },
        workshop: workshop_data,
    }
}

pub async fn download_image(client: Client, url: String) -> Result<Handle, Error> {
    let image_bytes = client.get(url).send().await?.bytes().await?;
    let image = image::load_from_memory(&image_bytes).unwrap();
    let resized = resize(&image, APP_IMAGES_SIZE, APP_IMAGES_SIZE);
    Ok(to_handle(&resized))
}
