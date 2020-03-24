use std::env::set_current_dir;
use std::fs::File;
use std::path::PathBuf;

use cursive::Cursive;

use crate::state::State;
use crate::update_window::update_window;

pub fn change_dir(s: &mut Cursive, name: &str) -> Result<(), String> {
    let metadata = File::open(PathBuf::from(name))
        .or_else(|e| Err(e.to_string()))?
        .metadata()
        .or_else(|e| Err(e.to_string()))?;

    if !metadata.is_dir() {
        return Err("Not a directory".to_string());
    }

    let show_hidden = s.user_data::<State>().unwrap().show_hidden;

    set_current_dir(PathBuf::from(name)).or_else(|e| Err(e.to_string()))?;
    update_window(s, show_hidden);

    Ok(())
}

pub fn refresh(s: &mut Cursive) -> Result<(), String> {
    change_dir(s, ".")
}

pub fn cd_parent(s: &mut Cursive) -> Result<(), String> {
    change_dir(s, "..")
}
