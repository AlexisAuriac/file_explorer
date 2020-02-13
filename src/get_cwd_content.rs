use std::fs::{read_dir, DirEntry};
use std::io;

fn hide_hidden_files(entries: Vec<DirEntry>) -> Vec<DirEntry> {
    entries
        .into_iter()
        .filter(|s| match s.file_name().to_str() {
            Some(s) if !s.starts_with('.') => true,
            _ => false,
        })
        .collect::<Vec<_>>()
}

fn entry_to_string_metadata(entries: Vec<DirEntry>) -> Vec<String> {
    entries
        .iter()
        .map(|entry| entry.file_name().to_str().unwrap().to_string())
        .collect::<Vec<_>>()
}

fn insert_current_parent_dir(entries: &mut Vec<String>) {
    entries.insert(0, "..".to_string());
    entries.insert(0, ".".to_string());
}

pub fn get_cwd_content(path: &str, show_hidden: bool) -> Result<Vec<String>, io::Error> {
    let entries = read_dir(path)?.collect::<Result<Vec<_>, io::Error>>()?;

    let entries = if !show_hidden {
        hide_hidden_files(entries)
    } else {
        entries
    };

    let mut entries = entry_to_string_metadata(entries);
    insert_current_parent_dir(&mut entries);

    Ok(entries)
}
