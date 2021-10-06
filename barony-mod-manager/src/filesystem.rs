use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn barony_default_dir() -> PathBuf {
    // TODO: Depends on the OS
    Path::new("~/.barony").to_owned()
}

#[derive(Serialize, Deserialize)]
pub struct SettingsPersistance {
    pub barony_directory_path: Option<String>,
    pub steam_api_key: Option<String>,
}

pub fn persist_settings(settings: SettingsPersistance) {
    // TODO: Create on_exit hook and run this inside it
    if let Some(user_data_dir) = dirs::data_dir() {
        let mod_manager_data_dir = user_data_dir.join("barony-mod-manager");
        fs::create_dir_all(&mod_manager_data_dir).unwrap();
        let json = serde_json::to_string(&settings).unwrap();
        fs::write(mod_manager_data_dir.join("settings.json"), json).unwrap();
    }
}

pub fn load_persisted_settings() -> SettingsPersistance {
    let mut settings = SettingsPersistance {
        barony_directory_path: None,
        steam_api_key: None,
    };

    if let Some(user_data_dir) = dirs::data_dir() {
        let mod_manager_data_dir = user_data_dir.join("barony-mod-manager");
        match fs::read_to_string(mod_manager_data_dir.join("settings.json")) {
            Ok(content) => {
                let content: SettingsPersistance = serde_json::from_str(content.as_str()).unwrap();
                settings = content;
            }
            // TODO: Maybe use logger or something
            Err(_) => println!("Could not read persistence file."),
        }
    }

    settings
}
