use std::io;

use argh::FromArgs;
use cursive::event::Key;
use cursive::theme::{Effect, Style};
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::{Dialog, EditView, SelectView};
use cursive::Cursive;
use enumset::EnumSet;

mod change_dir;
mod exec_command;
mod get_command;
mod get_cwd_content;
mod get_dir_fullname;
mod update_window;

use change_dir::change_dir;
use exec_command::exec_command;
use get_command::get_command;
use get_cwd_content::get_cwd_content;
use get_dir_fullname::get_dir_fullname;

#[derive(FromArgs, Debug)]
/// A terminal file explorer
struct Args {
    /// show hidden files
    #[argh(switch, short = 's')]
    pub show_hidden: bool,
    /// directory to start in
    #[argh(option)]
    pub starting_dir: Option<String>,
}

fn initialize_content_select(s: &mut Cursive, show_hidden: bool) -> io::Result<()> {
    let dir_style = Style {
        effects: EnumSet::only(Effect::Italic).union(EnumSet::only(Effect::Bold)),
        color: None,
    };

    let entries = get_cwd_content(show_hidden)?;
    let mut select = SelectView::<String>::new();

    for (name, is_dir) in entries {
        if is_dir {
            select.add_item(StyledString::styled(&name, dir_style), name);
        } else {
            select.add_item(name.clone(), name);
        }
    }

    let select = Dialog::around(
        select
            .on_submit(move |s, name| change_dir(s, name, show_hidden))
            .with_name("select")
            .scrollable()
            .fixed_size((30, 20)),
    )
    .title(get_dir_fullname("."))
    .with_name("dialog");

    s.add_layer(select);

    Ok(())
}

fn initialize_events(s: &mut Cursive, show_hidden: bool) {
    s.add_global_callback('q', |s| s.quit());
    s.add_global_callback(' ', get_command);
    s.add_global_callback(Key::Backspace, move |s| change_dir(s, "..", show_hidden));
}

fn get_cmd(s: &mut Cursive) -> Option<String> {
    let edit = s.find_name::<EditView>("edit_cmd")?;

    Some(edit.get_content().to_string())
}

fn get_selected_file(s: &mut Cursive) -> String {
    s.find_name::<SelectView<String>>("select")
        .unwrap()
        .selection()
        .unwrap()
        .to_string()
}

fn main() -> io::Result<()> {
    let args: Args = argh::from_env();

    let mut siv = Cursive::default();

    initialize_content_select(&mut siv, args.show_hidden)?;
    initialize_events(&mut siv, args.show_hidden);

    siv.run();

    let cmd = get_cmd(&mut siv);

    if cmd.is_none() {
        return Ok(());
    }

    let file = get_selected_file(&mut siv);

    drop(siv);

    exec_command(&file, &cmd.unwrap());

    Ok(())
}
