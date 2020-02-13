use std::fs::{read_dir, DirEntry, File, Metadata};
use std::io;
use std::path::PathBuf;

fn hide_hidden_files(entries: Vec<DirEntry>) -> Vec<DirEntry> {
    entries
        .into_iter()
        .filter(|s| match s.file_name().to_str() {
            Some(s) if !s.starts_with('.') => true,
            _ => false,
        })
        .collect::<Vec<_>>()
}

fn entry_to_string_metadata(entries: Vec<DirEntry>) -> Vec<(String, Metadata)> {
    entries
        .iter()
        .map(|entry| {
            (
                entry.file_name().to_str().unwrap().to_string(),
                entry.metadata().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn insert_current_parent_dir(entries: &mut Vec<(String, Metadata)>) {
    entries.insert(
        0,
        (
            "..".to_string(),
            File::open(PathBuf::from("..")).unwrap().metadata().unwrap(),
        ),
    );
    entries.insert(
        0,
        (
            ".".to_string(),
            File::open(PathBuf::from(".")).unwrap().metadata().unwrap(),
        ),
    );
}

pub fn get_cwd_content(path: &str, hide: bool) -> Result<Vec<(String, Metadata)>, io::Error> {
    let entries = read_dir(path)?.collect::<Result<Vec<_>, io::Error>>()?;

    let entries = if hide {
        hide_hidden_files(entries)
    } else {
        entries
    };

    let mut entries = entry_to_string_metadata(entries);
    insert_current_parent_dir(&mut entries);

    Ok(entries)
}
