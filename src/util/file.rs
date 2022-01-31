use std::fs::{File, OpenOptions, remove_dir_all};
use std::io::{Read, Write};

use super::Result;

pub fn get(path: String, create: bool, truncate: bool) -> Option<File> {
    match OpenOptions::new()
    .read(true)
    .write(true)
    .create(create)
    .truncate(truncate)
    .open(path) {
        Ok(result) => Some(result),
        Err(_) => None
    }
}

pub fn exists(path: String) -> bool {
    match get(path, false, false) {
        Some(_) => {
            true
        },
        None => false
    }
}

pub fn read(path: String) -> Option<String> {
    match get(path, false, false) {
        Some(mut file) => {
            let mut data = String::new();
            if file.read_to_string(&mut data).is_ok() {
                Some(data)
            } else {
                None
            }
        },
        None => None
    }
}

pub fn write(path: String, data: String) -> Result {
    match get(path, true, true) {
        Some(mut file) => super::to_result(file.write_all(&data.as_bytes())),
        None => super::Err()
    }
}

pub fn delete_folder(path: String) -> Result {
    super::to_result(remove_dir_all(path))
}
