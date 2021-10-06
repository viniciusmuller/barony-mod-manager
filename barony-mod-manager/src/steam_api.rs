// use iced::futures::executor::block_on;

// use std::future::Future;

use std::{io::Bytes, time::Duration};

use iced_native::executor::Tokio;
use rand::Rng;
use reqwest::{Client, Error};

use crate::{
    data::{
        BaronyMod, SteamApiResponse, SteamWorkshopMod, SteamWorkshopModResponse, SteamWorkshopTotal,
    },
    filesystem::{is_mod_active, is_mod_downloaded},
    widgets::Message,
};
// use std::collections::HashMap;

// static BARONY_APP_ID: String = "371970".;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

static BARONY_APP_ID: u64 = 371970;

// pub async fn get_mods_quantity(steam_key: String) -> u64 {}

// pub async fn get_mods(client: Client, steam_key: String) -> Result<u64, String> {
//     // let first_mod = request_workshop_page(&client, steam_key, 1).await?;
//     let total = get_total_mods(client, steam_key).await?;
//     Ok(total)
//     // (1..=total).map(|n| async { request_workshop_page(client, steam_key, n) })
// }

pub async fn get_total_mods(client: Client, steam_key: String) -> Result<u64, String> {
    let params = map!(
        "key" => steam_key,
        "appid" => BARONY_APP_ID.to_string()
    );

    let response = client
        .get("https://api.steampowered.com/IPublishedFileService/QueryFiles/v1")
        .query(&params)
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() != 403 {
                let content = response.text().await.unwrap();
                let content = content.as_str();
                let json: SteamApiResponse<SteamWorkshopTotal> =
                    serde_json::from_str(content).unwrap();
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

pub async fn get_barony_mod() {}

pub async fn get_workshop_item(
    client: Client,
    steam_key: String,
    mod_number: u64,
) -> Result<BaronyMod, String> {
    let params = map!(
        "key" => steam_key,
        "appid" => BARONY_APP_ID.to_string(),
        "return_tags" => true.to_string(),
        "return_vote_data" => true.to_string(),
        "strip_description_bbcode" => true.to_string(),
        "page" => mod_number.to_string()
    );

    // Random sleep so that steam doesn't rate limit us for slapping 100+ concurrent
    // requests on them.
    let duration = Duration::from_millis(mod_number * 30);
    async_std::task::sleep(duration).await;

    let response = client
        .get("https://api.steampowered.com/IPublishedFileService/QueryFiles/v1")
        .query(&params)
        .send()
        .await;

    let content = match response {
        Ok(response) => response.text().await.unwrap(),
        Err(_error) => return Err("Failed to get mod data.".to_string()),
    };
    // let content =
    let mut mod_: SteamApiResponse<SteamWorkshopModResponse> =
        serde_json::from_str(content.as_str()).unwrap();

    let steam_mod = mod_.response.mods.pop().unwrap();

    let mut barony_mod = BaronyMod {
        is_downloaded: is_mod_downloaded(steam_mod.title.clone()),
        is_active: is_mod_active(steam_mod.title.clone()),
        workshop: steam_mod.clone(),
        image_binary: None,
    };

    barony_mod.image_binary = if !steam_mod.preview_url.is_empty() {
        match download_image(client, steam_mod.preview_url.clone()).await {
            Ok(image_bytes) => Some(image_bytes),
            Err(_err) => None,
        }
    } else {
        None
    };

    Ok(barony_mod)
}

pub async fn download_image(client: Client, url: String) -> Result<Vec<u8>, Error> {
    let image_bytes = client.get(url).send().await?.bytes().await?;
    Ok(image_bytes.to_vec())
}

// pub async fn steam_request(steam_key: String) -> Result<Vec<SteamWorkshopMod>, String> {

//     // let client = reqwest::Client::new();
// }
// dbg!(&response);
// match response {
//     Ok(result) => {
//         if result.status() == 403 {
//             return Err("Not authenticated. Invalid Steam API key.".to_string());
//         }

// let text = result.text().await;
// match text {
//     Ok(data)ii => {
//         let json: SteamApiResponse = serde_json::from_str(data.as_str()).unwrap();
//         // dbg!(json);
//         // Ok(json)
//     }
//     Err(_err) => Err("failure".to_string()),
// }
// }
// Err(_error) => Err("fatal failure".to_string()),
// }
// // dbg!(&response);
// // response
// }

// async fn get_mods_data(steam_key: String) -> Result<Vec<SteamWorkshopMod>, &'static str> {
//     let mut params = map!(
//         "key" => steam_key,
//         "appid" => BARONY_APP_ID
//     );

//     let client = reqwest::Client::new();

//     let response = block_on(|| {
//         client
//             .get("https://api.steampowered.com/IPublishedFileService/QueryFiles/v1")
//             .form(&params)
//             .send()
//     });

//     dbg!(response);
//     if let Ok(response) = response {
//         dbg!(response);
//         return Err("yeah");
//     } else {
//         return Err("Could not request Steam successfully. Please make sure that the provided steam key is valid.");
//     }
// }
