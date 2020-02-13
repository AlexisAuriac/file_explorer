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

fn entry_to_string(entries: Vec<DirEntry>) -> Vec<(String, bool)> {
    let mut entries = entries
        .iter()
        .map(|entry| {
            (
                entry.file_name().to_str().unwrap().to_string(),
                entry.metadata().unwrap().is_dir(),
            )
        })
        .collect::<Vec<_>>();
    entries.sort_by(|a, b| a.cmp(b));

    entries
}

fn insert_current_parent_dir(entries: &mut Vec<(String, bool)>) {
    entries.insert(0, ("..".to_string(), true));
    entries.insert(0, (".".to_string(), true));
}

pub fn get_cwd_content(show_hidden: bool) -> Result<Vec<(String, bool)>, io::Error> {
    let entries = read_dir(".")?.collect::<Result<Vec<_>, io::Error>>()?;

    let entries = if !show_hidden {
        hide_hidden_files(entries)
    } else {
        entries
    };

    let mut entries = entry_to_string(entries);
    insert_current_parent_dir(&mut entries);

    Ok(entries)
}
