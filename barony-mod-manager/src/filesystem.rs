use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

#[derive(Serialize, Deserialize)]
pub struct SettingsPersistance {
    pub barony_directory_path: Option<String>,
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

pub fn barony_dir_valid(dir: &str) -> bool {
    let barony_path = Path::new(dir);
    let barony_mods_path = barony_path.join("mods/");

    barony_path.exists()
        && barony_path.is_dir()
        && barony_mods_path.exists()
        && barony_mods_path.is_dir()
}

pub fn load_persisted_settings() -> SettingsPersistance {
    let mut settings = SettingsPersistance {
        barony_directory_path: None,
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

pub fn is_mod_downloaded(barony_path: &str, mod_title: &str) -> bool {
    if mod_title.is_empty() {
        return false;
    }

    // Add a trailing slash since mods lies inside folders
    let mod_path = format!("{}/", mod_title);
    Path::new(barony_path).join("mods/").join(mod_path).exists()
}

pub fn write_mod_to_disk(
    barony_path: String,
    mod_title: String,
    zip_bytes: Vec<u8>,
) -> Result<(), std::io::Error> {
    let mod_title_clean = clean_filename(&mod_title);
    let mod_folder = Path::new(&barony_path)
        .join("mods/")
        .join(format!("{}/", mod_title_clean));

    let cursor = std::io::Cursor::new(zip_bytes);
    let mut archive = zip::ZipArchive::new(cursor).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => mod_folder.join(path.to_owned()),
            None => continue,
        };

        if (file.name()).ends_with('/') {
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
    }

    Ok(())
}

pub fn delete_mod_from_disk(barony_path: &str, mod_title: &str) -> Result<(), std::io::Error> {
    let foldername = clean_filename(mod_title);
    let mod_path = Path::new(barony_path).join("mods/").join(foldername);
    std::fs::remove_dir_all(mod_path)
}

// This removes invalid filename characters that would make the program fail with
// an OS error while trying to write the mod folder to disk.
fn clean_filename(filename: &str) -> String {
    filename.replace(&['<', '>', ':', '/', '\\', '|', '?', '*'][..], "")
}
