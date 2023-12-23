// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use serde::{Deserialize, Serialize};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_file(folder_path: &str) -> Result<Vec<FileInfo>> {
    let paths = fs::read_dir(folder_path);
    if paths.is_err() {
        return Result::error(format!("读取路径{}错误 {:?}", folder_path, paths.unwrap_err()));
    }
    let paths = paths.unwrap();

    let mut file_info_vec = vec![];
    for path in paths {
        let path = path.unwrap();
        let file_size = get_file_size(&path);
        file_info_vec.push(FileInfo {
            file_name: path.file_name().into_string().unwrap(),
            file_path: path.path().to_str().unwrap().to_string(),
            file_size,
        });
    }
    Result::success(Some(file_info_vec))
}

#[tauri::command]
fn get_file_parent_path(folder_path: &str) -> Result<String> {
    let path = Path::new(folder_path);

    let parent = path.parent();
    if parent.is_none() {
        return Result::error(format!("没有父级"));
    }
    Result::success(Some(parent.unwrap().to_str().unwrap().to_string()))
}

fn get_file_size(path: &DirEntry) -> u64 {
    if path.file_type().unwrap().is_file() {
        return path.metadata().unwrap().len();
    }
    let paths = fs::read_dir(path.path());
    if paths.is_err() {
        println!("读取文件失败 = {:?}", paths);
        return 0;
    }
    let paths = paths.unwrap();
    let mut size = 0;
    for path in paths {
        let path = path.unwrap();
        if path.file_type().unwrap().is_file() {
            size = size + path.metadata().unwrap().len();
        } else {
            size = size + get_file_size(&path);
        }
    }
    size
}

#[derive(Serialize, Deserialize, Debug)]
struct Result<T> {
    pub(crate) success: bool,
    pub(crate) msg: Option<String>,
    pub(crate) data: Option<T>,
}

impl<T> Result<T> {
    pub fn success(data: Option<T>) -> Self {
        Result {
            success: true,
            msg: None,
            data,
        }
    }

    pub fn error(msg: String) -> Self {
        Result {
            success: false,
            msg: Some(msg),
            data: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FileInfo {
    pub(crate) file_name: String,
    pub(crate) file_path: String,
    pub(crate) file_size: u64,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,list_file,get_file_parent_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
