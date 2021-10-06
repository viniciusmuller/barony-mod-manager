use std::path::{Path, PathBuf};

fn barony_default_dir() -> PathBuf {
    // TODO: Depends on the OS
    Path::new("~/.barony").to_owned()
}
