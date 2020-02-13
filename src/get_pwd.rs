use std::fs::canonicalize;
use std::path::PathBuf;

pub fn get_pwd() -> String {
    canonicalize(PathBuf::from("."))
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
