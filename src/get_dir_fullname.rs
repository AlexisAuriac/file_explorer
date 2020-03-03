use std::fs::canonicalize;
use std::path::PathBuf;

pub fn get_dir_fullname(dir: &str) -> String {
    canonicalize(PathBuf::from(dir))
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
