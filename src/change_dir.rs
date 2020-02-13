use std::env::set_current_dir;
use std::fs::File;
use std::path::PathBuf;

use cursive::theme::{Effect, Style};
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::{Dialog, EditView, SelectView};
use cursive::Cursive;
use enumset::EnumSet;

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

fn update_content(s: &mut Cursive, show_hidden: bool) {
    let entries = get_cwd_content(".", show_hidden).unwrap();
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();

    let dir_style = Style {
        effects: EnumSet::only(Effect::Italic).union(EnumSet::only(Effect::Bold)),
        color: None,
    };

    select.clear();

    for (name, is_dir) in entries {
        if is_dir {
            select.add_item(StyledString::styled(&name, dir_style), name);
        } else {
            select.add_item(name.clone(), name);
        }
    }
}

pub fn change_dir(s: &mut Cursive, name: &str, show_hidden: bool) {
    let metadata = File::open(PathBuf::from(name)).unwrap().metadata().unwrap();

    if !metadata.is_dir() {
        get_command_edit_view(s);
        return;
    }

    set_current_dir(PathBuf::from(name)).unwrap();

    update_title(s);
    update_content(s, show_hidden);
}
