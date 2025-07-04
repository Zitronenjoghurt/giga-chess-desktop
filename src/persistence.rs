use crate::app::GigaChessAppPersist;
use directories::ProjectDirs;
use log::error;
use std::path::PathBuf;

pub mod color;

pub trait PersistentObject {
    type PersistentType;

    fn save_state(&self) -> Self::PersistentType;
    fn load_from_state(state: Self::PersistentType) -> Self;
}

fn get_project_dirs() -> ProjectDirs {
    ProjectDirs::from("io.github", "zitronenjoghurt", "giga-chess").unwrap()
}

fn get_save_dir() -> PathBuf {
    get_project_dirs().data_dir().to_path_buf()
}

fn get_save_file_path() -> PathBuf {
    get_save_dir().join("save.json")
}

pub fn persist(state: GigaChessAppPersist) {
    let directory = get_save_dir();
    if !directory.exists() {
        std::fs::create_dir_all(&directory).unwrap();
    }

    let save_path = get_save_file_path();
    let Ok(data) = serde_json::to_string_pretty(&state) else {
        error!("Failed to serialize app state to JSON");
        return;
    };

    let write_result = std::fs::write(save_path, data);
    if let Err(e) = write_result {
        error!("Failed to write save file: {e}");
    }
}

pub fn restore() -> Option<GigaChessAppPersist> {
    let save_path = get_save_file_path();
    if !save_path.exists() {
        return None;
    }

    let Ok(data) = std::fs::read_to_string(save_path) else {
        panic!("Failed to read save file");
    };

    let Ok(state) = serde_json::from_str(&data) else {
        panic!("Failed to deserialize save file");
    };

    Some(state)
}
