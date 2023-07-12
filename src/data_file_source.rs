use std::{fs, io::ErrorKind, path::PathBuf};

use crate::prelude::*;
fn get_data_source_path() -> AppResult<PathBuf> {
    if cfg!(debug_assertions) {
        let to_dev_folder = PathBuf::from(constants::PROJECT_ROOT).join(constants::DEV_DATA_FOLDER);
        fs::create_dir_all(&to_dev_folder)?;
        Ok(to_dev_folder.join(constants::DATA_SOURCE_NAME))
    } else {
        todo!("Not implemented for productio");
    }
}

pub fn provide_data() -> AppResult<Todos> {
    let path = get_data_source_path()?;

    match fs::read_to_string(&path) {
        Ok(content) => {
            let data = serde_json::from_str(&content)?;
            Ok(data)
        }
        Err(error) => {
            if let ErrorKind::NotFound = error.kind() {
                Ok(Default::default())
            } else {
                Err(anyhow!("Could not load Todos from path: {:?}", path))
            }
        }
    }
}
pub fn save_data(to_save: &Todos) -> AppResult {
    let path = get_data_source_path()?;

    let to_str = serde_json::to_string_pretty(to_save)?;

    fs::write(path, to_str)?;

    Ok(())
}
