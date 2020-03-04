use std::io;

use argh::FromArgs;
use cursive::event::Key;

use cursive::traits::*;

use cursive::views::{Dialog, EditView, SelectView};
use cursive::Cursive;

mod change_dir;
mod exec_command;
mod get_command;
mod get_cwd_content;
mod get_dir_fullname;
mod state;
mod update_window;

use change_dir::{cd_parent, change_dir, refresh};
use exec_command::exec_command;
use get_command::get_command;

use get_dir_fullname::get_dir_fullname;
use state::State;

#[derive(FromArgs, Debug)]
/// A terminal file explorer
struct Args {
    /// show hidden files
    #[argh(switch, short = 's')]
    pub show_hidden: bool,
    /// directory to start in
    #[argh(option, short = 'S')]
    pub start: Option<String>,
}

fn initialize_content_select(s: &mut Cursive, start: Option<String>) -> io::Result<()> {
    let select = SelectView::<String>::new();

    let select = Dialog::around(
        select
            .on_submit(move |s, name| change_dir(s, name))
            .with_name("select")
            .scrollable()
            .fixed_size((30, 20)),
    )
    .title(get_dir_fullname("."))
    .with_name("dialog");

    s.add_layer(select);

    if let Some(dir) = start {
        change_dir(s, &dir);
    } else {
        refresh(s);
    }

    Ok(())
}

fn initialize_events(s: &mut Cursive) {
    s.add_global_callback('q', |s| s.quit());
    s.add_global_callback(' ', get_command);

    s.add_global_callback(Key::Backspace, |s| cd_parent(s));

    s.add_global_callback('s', move |s| {
        let state = s.user_data::<State>().unwrap();

        state.show_hidden = !state.show_hidden;
        refresh(s);
    });
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

    siv.set_user_data(State::new(args.show_hidden));
    initialize_content_select(&mut siv, args.start)?;
    initialize_events(&mut siv);

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
