#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use fs_extra::dir::get_size;
use serde::{Deserialize, Serialize};
use std::io;
use std::{fs, path::PathBuf};
use walkdir::WalkDir;
use image;
use image_base64;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Paths {
    path: String,
    size: u64,
    len: u64,
    age: u64,
}

#[tauri::command]
fn get_paths(dir_path: &str) -> Vec<Paths> {
    let mut directory = PathBuf::new();
    directory.push(dir_path);

    let mut pathes: Vec<Paths> = Vec::new();

    for entry in fs::read_dir(&directory).unwrap() {
        let path = entry.unwrap();
        pathes.push(combine(&path));
    }

    pathes

    /*
        WalkDir::new(dir_path)
            .into_iter()
            .filter_entry(|e| is_not_hidden(e))
            .filter_map(|v| v.ok())
            .for_each(|x| sorted.push(combine(&x)));
    */

    /*
    println!("The entries in {:?} sorted ascendingly by size:", directory);

    for x in &pathes {
        println!(
            "{:?} size is: {:?} bytes, len: {}, age: {}",
            x.path, x.size, x.len, x.age
        );
    }
    println!("\n\n");


    println!(
        "The entries in {:?} pathsizes ascendingly by size:",
        directory
    );
    */
}
fn combine(x: &std::fs::DirEntry) -> Paths {
    let path = x.path();
    let metadata = fs::metadata(&path).unwrap();
    let created = metadata.created().unwrap().elapsed().unwrap().as_secs();
    let mut len = 0;

    let path_str = path.to_str().unwrap().to_string();
    WalkDir::new(path_str)
        .into_iter()
        .filter_map(|v| v.ok())
        .for_each(|_| len += 1);

    let combined = Paths {
        path: path.file_name().unwrap().to_str().unwrap().to_string(),
        size: get_size(&path).unwrap_or(0),
        age: created,
        len: len - 1,
    };

    combined
}

#[tauri::command]
fn sort_by_size(pathes_ref: &str) -> Vec<Paths> {
    let mut pathes:Vec<Paths> = serde_json::from_str(&pathes_ref).unwrap();

    //let mut pathes = pathes_ref.clone();
    let length = pathes.len();
    for i in 0..(length - 1) {
        for j in 0..(length - i - 1) {
            if pathes[j].size > pathes[j + 1].size {
                pathes.swap(j, j + 1);
            }
        }
    }

    pathes
}
#[tauri::command]
fn sort_by_age(pathes_ref: Vec<Paths>) -> Vec<Paths> {
    let mut pathes = pathes_ref.clone();
    let length = pathes.len();
    for i in 0..(length - 1) {
        for j in 0..(length - i - 1) {
            if pathes[j].age > pathes[j + 1].age {
                pathes.swap(j, j + 1);
            }
        }
    }

    pathes
}
#[tauri::command]
fn sort_by_len(pathes_ref: Vec<Paths>) -> Vec<Paths> {
    let mut pathes = pathes_ref.clone();
    let length = pathes.len();
    for i in 0..(length - 1) {
        for j in 0..(length - i - 1) {
            if pathes[j].len > pathes[j + 1].len {
                pathes.swap(j, j + 1);
            }
        }
    }

    pathes
}

#[tauri::command]
fn delete(dir_fil_path: &str) {
    let mut directory = PathBuf::new();
    directory.push(&dir_fil_path);
    println!("{:?}",&dir_fil_path);
    if directory.is_dir() {
        fs::remove_dir_all(dir_fil_path).unwrap();
    } else {
        fs::remove_file(dir_fil_path.to_string()).unwrap();
    }
}

#[tauri::command]
fn to_image(base: String) {
    let x = "/home/khalilo/Downloads/output".to_string();
    let image = image_base64::from_base64(base);
    let path = Path::new(&x);

    match image::load_from_memory_with_format(&image, image::ImageFormat::Png) {
        Ok(_img) => {
            println!("input in png");
            std::fs::write(path, image).unwrap();
        }
        Err(_) => {
            println!("input is not png");
        }
    }
}
/*
#[tauri::command]
pub fn ter_open(path: String) {
    let output = if cfg!(target_os = "windows") {
        Command::new("cd")
            .args(["C:\\Users\\Mohamed\\Downloads\\Documents"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("PUT CD.EXE PATH")
            .arg(path)
            .output() // try spawn
            .expect("failed to execute process")
    };
    let hello = output.stdout;
    println!("{:?}", hello);
}
*/
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_paths, delete, to_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


}

// https://docs.rs/json/latest/json/
// https://crates.io/crates/serde_json