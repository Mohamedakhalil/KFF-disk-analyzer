/* 
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde_json;
use fs_extra::dir::get_size;
use serde::{Deserialize, Serialize};
use std::{fs, error, io::{self, ErrorKind}, path::Path, path::PathBuf};


#[derive(Serialize, Deserialize, Debug)]
struct PathSizes {
    path: String,
    size: u64,
    contents: u64,
    
}

#[tauri::command]
fn sort_by_size(dir_path: &str) -> Vec<PathSizes> {
    let mut directory = PathBuf::new();
    directory.push(dir_path);
    //println!("{:?}", &directory);
    let mut pathsizes: Vec<PathSizes> = Vec::new();

    for entry in fs::read_dir(&directory).unwrap() {
        let path = entry.unwrap();
        pathsizes.push(combine(&path));
    }

    //let pathsizes_json = serde_json::to_string(&pathsizes).unwrap();
    //println!("{}", &pathsizes_json);
    pathsizes
}

fn combine(x: &std::fs::DirEntry) -> PathSizes {
    let path = x.path();
    //println!("{:?}", &path);

    let mut size = get_size(&path);
    if get_size(&path) == Ok(){
        size = get_size(&path).unwrap();
    }
    

    let combined = PathSizes {
        path: path.file_name().unwrap().to_str().unwrap().to_string(),
        size: if (get_size(path).unwrap() == None)? 0: ,
    };

    combined
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sort_by_size])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


}
*/
// https://docs.rs/json/latest/json/
// https://crates.io/crates/serde_json
