use std::env::set_current_dir;
use std::fs::File;
use std::path::PathBuf;

use cursive::views::Dialog;
use cursive::Cursive;

use crate::state::State;
use crate::update_window::update_window;

pub fn change_dir(s: &mut Cursive, name: &str) {
    let metadata = File::open(PathBuf::from(name)).unwrap().metadata().unwrap();

    if !metadata.is_dir() {
        s.add_layer(Dialog::info("Not a directory"));
        return;
    }

    let show_hidden = s.user_data::<State>().unwrap().show_hidden;

    set_current_dir(PathBuf::from(name)).unwrap();
    update_window(s, show_hidden);
}

pub fn refresh(s: &mut Cursive) {
    change_dir(s, ".");
}

pub fn cd_parent(s: &mut Cursive) {
    change_dir(s, "..");
}
