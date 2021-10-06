// use iced::futures::executor::block_on;

// use crate::data::SteamWorkshopMod;
// use std::collections::HashMap;

// static BARONY_APP_ID: u64 = 371970;

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
