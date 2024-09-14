mod db;
mod models;
mod commands;
mod config;

use std::path::PathBuf;
use std::sync::Mutex;
use crate::commands::{create_store, open_store, get_store_list, create_space, get_space, get_space_list, create_bin, get_bin_list, create_item, get_item_list};
use crate::config::initialize_config;
use config::ConfigState;

fn main() {
    let config_state = ConfigState {
        config_path: PathBuf::new(),
    };

    tauri::Builder::default()
        .manage(Mutex::new(config_state))
        .setup(initialize_config)
        .invoke_handler(tauri::generate_handler![create_store, open_store, get_store_list, create_space, get_space, get_space_list, create_bin, get_bin_list, create_item, get_item_list])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
