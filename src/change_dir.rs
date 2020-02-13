use std::env::set_current_dir;
use std::fs::File;
use std::path::PathBuf;

use cursive::traits::*;
use cursive::views::{Dialog, EditView, SelectView};
use cursive::Cursive;

use crate::get_cwd_content::get_cwd_content;
use crate::get_pwd::get_pwd;

fn get_command_edit_view(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(|s, _| s.quit())
                .with_name("edit_cmd"),
        )
        .title("Exec command"),
    );
}

fn update_title(s: &mut Cursive) {
    let mut dialog = s.find_name::<Dialog>("dialog").unwrap();

    dialog.set_title(get_pwd());
}

fn update_content(s: &mut Cursive) {
    let entries = get_cwd_content(".", true).unwrap();
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();

    select.clear();
    select.add_all_str(entries);
}

pub fn change_dir(s: &mut Cursive, name: &str) {
    let metadata = File::open(PathBuf::from(name)).unwrap().metadata().unwrap();

    if !metadata.is_dir() {
        get_command_edit_view(s);
        return;
    }

    set_current_dir(PathBuf::from(name)).unwrap();

    update_title(s);
    update_content(s);
}
