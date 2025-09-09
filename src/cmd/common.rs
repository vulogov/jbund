extern crate log;
use std::fs;

pub fn get_file_from_relative_file(full_path: String) -> Option<String> {
    match fs::read_to_string(full_path) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}
