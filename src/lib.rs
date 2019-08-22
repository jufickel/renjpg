// Copyright (c) 2019 Juergen Fickel
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate exif;

use exif::{DateTime, Reader, Tag, Value};
use std::ffi::OsStr;
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use std::str;

pub fn rename_user_provided_files(
    file_paths: &Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    for file_path in file_paths {
        if is_jpeg_file(&file_path) {
            rename_jpeg(&file_path);
        } else {
            eprintln!("Ignoring '{}' because it is not a JPEG file!", file_path.display());
        }
    }

    Ok(())
}

fn is_jpeg_file(path: &PathBuf) -> bool {
    if !path.is_file() {
        return false;
    }

    let extension = path
        .extension()
        .and_then(OsStr::to_str)
        .map(str::to_lowercase);

    match extension {
        Some(extension) => "jpg" == extension || "jpeg" == extension,
        None => false,
    }
}

fn rename_jpeg(file_path: &PathBuf) {
    let filename = get_filename(&file_path);
    match get_original_date_time(&file_path).map(|date_time| format_date_time(&date_time)) {
        Some(ref date_time_string) if !is_prefixed(&filename, &date_time_string) => {
            rename(&filename, &date_time_string)
        }
        _ => {}
    }
}

pub fn rename_all_jpegs_in_dir(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir_entries = fs::read_dir(path)?;

    for dir_entry in dir_entries {
        let entry = dir_entry?;
        let entry_path = entry.path();
        if is_jpeg_file(&entry_path) {
            rename_jpeg(&entry_path);
        }
    }

    Ok(())
}

fn get_original_date_time(jpeg_file_path: &PathBuf) -> Option<DateTime> {
    let file = fs::File::open(jpeg_file_path).unwrap();
    if let Ok(reader) = Reader::new(&mut BufReader::new(&file)) {
        if let Some(date_time_field) = reader.get_field(Tag::DateTimeOriginal, false) {
            return match date_time_field.value {
                Value::Ascii(ref vec) if !vec.is_empty() => get_date_time_from_ascii(vec[0]),
                _ => None,
            };
        }
    }
    None
}

fn get_date_time_from_ascii(data: &[u8]) -> Option<DateTime> {
    let result: Result<DateTime, exif::Error> = DateTime::from_ascii(data);
    result.ok()
}

fn get_filename(jpeg_file_path: &PathBuf) -> &str {
    jpeg_file_path.file_name().and_then(OsStr::to_str).unwrap()
}

fn format_date_time(date_time: &DateTime) -> String {
    format!(
        "{}-{:02}-{:02}_{:02}{:02}{:02}",
        date_time.year,
        date_time.month,
        date_time.day,
        date_time.hour,
        date_time.minute,
        date_time.second
    )
}

fn is_prefixed(filename: &str, prefix: &str) -> bool {
    filename.starts_with(prefix)
}

fn rename(old_filename: &str, formatted_date_time: &str) {
    let new_filename = get_new_filename(old_filename, formatted_date_time);

    fs::rename(old_filename, new_filename).unwrap();
}

fn get_new_filename(jpeg_file_name: &str, formatted_date_time: &str) -> String {
    format!("{}_{}", formatted_date_time, jpeg_file_name)
}
