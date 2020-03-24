use std::fs::canonicalize;
use std::path::PathBuf;

pub fn get_file_full_name(dir: &str) -> Result<String, String> {
    match canonicalize(PathBuf::from(dir)) {
        Err(err) => Err(err.to_string()),
        Ok(path) => match path.to_str() {
            None => Err("Invalid UTF-8 characters in path".to_string()),
            Some(path) => Ok(path.to_string()),
        },
    }
}

pub fn get_cwd_full_name() -> Result<String, String> {
    get_file_full_name(".")
}
