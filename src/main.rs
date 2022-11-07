mod Interface;
mod executor;

extern crate serde_json;

use crate::Interface::run_interface;
use serde::{Deserialize, Serialize};
use std::env::var_os;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

fn main() {
    // We need to check firstly if there already exists a JSON file with the list of shortcuts and if not, we create it.
    let mut path = match executor::get_appdata() {
        Some(path) => path,
        _ => panic!("Couldn't find appdata path, your computer is truly fucked"),
    };

    path.push("BetterSeach");
    std::fs::create_dir(&path).ok();
    path.push("shortcuts.json");

    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let mut shortcut_file = File::create(&path).expect("Something has gone terribly wrong :)");

    //Next we will need to create a new CLI interface for the user to interact with. This will allow them to add, remove, edit, search, and run their shortcuts.
    run_interface(path);
}
