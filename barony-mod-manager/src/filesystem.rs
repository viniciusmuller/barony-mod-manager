use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

// fn barony_default_dir() -> PathBuf {
//     // TODO: Depends on the OS
//     Path::new("~/.barony").to_owned()
// }

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

pub fn barony_dir_valid(dir: &String) -> bool {
    let barony_path = Path::new(dir);
    barony_path.exists() && barony_path.is_dir()
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

pub fn is_mod_downloaded(barony_path: &String, mod_title: &String) -> bool {
    Path::new(barony_path).join(mod_title).exists()
}

// TODO: Should activating/deactivating mods be a future?

// pub fn is_mod_active(barony_path: &String, mod_id: &String) -> bool {
//     false
// }

// // All of those will be `Results`
// pub fn activate_mod(barony_path: &String, mod_id: &String) {
//     // Move the mod from the inactive_mods/ folder to the mods/ folder
// }

// pub fn deactivate_mod(barony_path: &String, mod_id: &String) {
//     // Move the mod from the mods/ folder to the inactive_mods/ folder
//     let mod_path = Path::new(barony_path).join("/mods").join(mod_id);
//     let inactive_mod_path = Path::new(barony_path).join("/inactive_mods").join(mod_id);
// }

pub fn write_mod_to_disk(
    barony_path: String,
    mod_title: String,
    zip_bytes: Vec<u8>,
) -> Result<(), std::io::Error> {
    let mod_folder = Path::new(&barony_path)
        .join("mods/")
        .join(format!("{}/", mod_title));

    let cursor = std::io::Cursor::new(zip_bytes);
    let mut archive = zip::ZipArchive::new(cursor).unwrap();

    Ok(for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => mod_folder.join(path.to_owned()),
            None => continue,
        };

        if (&*file.name()).ends_with('/') {
            // println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    })
}

pub fn delete_mod_from_disk(
    barony_path: &String,
    mod_title: &String,
) -> Result<(), std::io::Error> {
    let mod_path = Path::new(barony_path).join("mods/").join(mod_title);
    std::fs::remove_dir_all(mod_path)
}
