use cursive::theme::{Effect, Style};
use cursive::utils::markup::StyledString;
use cursive::views::{Dialog, SelectView};
use cursive::Cursive;
use enumset::EnumSet;

use crate::get_cwd_content::get_cwd_content;
use crate::get_dir_fullname::get_dir_fullname;

fn update_title(s: &mut Cursive) {
    let mut dialog = s.find_name::<Dialog>("dialog").unwrap();

    dialog.set_title(get_dir_fullname("."));
}

fn update_content(s: &mut Cursive, show_hidden: bool) {
    let entries = get_cwd_content(show_hidden).unwrap();
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

pub fn update_window(s: &mut Cursive, show_hidden: bool) {
    update_title(s);
    update_content(s, show_hidden);
}
