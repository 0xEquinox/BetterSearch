use std::{
    fs::{DirEntry, FileType, OpenOptions, ReadDir},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::Interface::Shortcut;

use serde_json;

pub(crate) fn search(search_val: &String, shortcuts: &mut Vec<Shortcut>) -> Vec<Shortcut> {
    shortcuts
        .iter()
        .filter(|shortcut| shortcut.name.to_lowercase().contains(search_val))
        .map(|shortcut| shortcut.clone())
        .collect()
}

pub fn pupulate_shortcuts(shortcut_file: PathBuf) {
    let search_dirs: Vec<PathBuf> = vec![
        PathBuf::from("C:/ProgramData/Microsoft/Windows/Start Menu/Programs"),
        get_appdata()
            .unwrap()
            .join("Microsoft/Windows/Start Menu/Programs"),
    ];

    let shortcut_json = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&shortcut_file)
        .expect("failed to open file");

    let mut shortcut_json = BufWriter::new(shortcut_json);

    let mut shortcuts: Vec<Shortcut> = Vec::new();

    for dir in search_dirs {
        let files = dir.read_dir().expect("Something has gone wrong");
        let mut new_shortcuts: Vec<Shortcut> = files
            .filter(|file| {
                file.as_ref()
                    .unwrap()
                    .file_name()
                    .into_string()
                    .unwrap()
                    .ends_with(".lnk")
            })
            .map(|file| {
                let length = file
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .into_string()
                    .unwrap()
                    .len();

                let mut file_name = file.as_ref().unwrap().file_name().into_string().unwrap();
                file_name.truncate(length - 4);
                Shortcut {
                    name: file_name,
                    path: file.unwrap().path().into_os_string().into_string().unwrap(),
                }
            })
            .collect();
        shortcuts.append(&mut new_shortcuts);
    }

    write!(
        shortcut_json,
        "{}",
        serde_json::to_string_pretty(&shortcuts).unwrap()
    )
    .ok();
}

pub fn get_appdata() -> Option<PathBuf> {
    std::env::var_os("APPDATA").map(PathBuf::from)
}
