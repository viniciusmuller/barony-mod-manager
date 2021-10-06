// use iced::futures::executor::block_on;

// use std::future::Future;

use reqwest::Response;

use crate::{data::SteamWorkshopMod, widgets::Message};
// use std::collections::HashMap;

static BARONY_APP_ID: u64 = 371970;

pub async fn steam_request(steam_key: String) -> String {
    // let mut params = map!(
    //     "key" => steam_key,
    //     "appid" => BARONY_APP_ID
    // );

    let client = reqwest::Client::new();
    dbg!(&client);

    let response = client
        .get("https://api.steampowered.com/IPublishedFileService/QueryFiles/v1")
        // .form(&params)
        .send()
        .await;

    dbg!(&response);
    match response {
        Ok(result) => {
            if result.status() != 403 {
                "success".to_string()
            } else {
                "failure".to_string()
            }
        }
        Err(_error) => "fatal failure".to_string(),
    }
    // dbg!(&response);
    // response
}

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

// macro_rules! map(
//     { $($key:expr => $value:expr),+ } => {
//         {
//             let mut m = ::std::collections::HashMap::new();
//             $(
//                 m.insert($key, $value);
//             )+
//             m
//         }
//      };
// );
