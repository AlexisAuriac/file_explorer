use cursive::reexports::enumset::EnumSet;
use cursive::theme::{ColorStyle, Effect, Style};
use cursive::utils::markup::StyledString;
use cursive::views::{Dialog, SelectView};
use cursive::Cursive;

use crate::dialog_res::dialog_res;
use crate::get_cwd_content::get_cwd_content;
use crate::get_file_full_name::get_cwd_full_name;

fn update_title(s: &mut Cursive) {
    let mut dialog = s.find_name::<Dialog>("dialog").unwrap();

    dialog.set_title(get_cwd_full_name().unwrap_or_else(|err| err));
}

fn update_content(s: &mut Cursive, show_hidden: bool) -> Result<(), String> {
    let entries = get_cwd_content(show_hidden).map_err(|err| err.to_string())?;
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();

    let dir_style = Style {
        effects: EnumSet::only(Effect::Italic).union(EnumSet::only(Effect::Bold)),
        color: ColorStyle::inherit_parent(),
    };

    select.clear();

    for (name, is_dir) in entries {
        if is_dir {
            select.add_item(StyledString::styled(&name, dir_style), name);
        } else {
            select.add_item(name.clone(), name);
        }
    }

    Ok(())
}

pub fn update_window(s: &mut Cursive, show_hidden: bool) {
    update_title(s);

    dialog_res(update_content(s, show_hidden), s);
}
