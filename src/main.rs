use std::io;

use cursive::traits::*;
use cursive::views::{Dialog, EditView, SelectView};
use cursive::Cursive;

mod change_dir;
mod exec_command;
mod get_cwd_content;
mod get_pwd;

use change_dir::change_dir;
use exec_command::exec_command;
use get_cwd_content::get_cwd_content;
use get_pwd::get_pwd;

fn initialize_content_select(s: &mut Cursive) -> io::Result<()> {
    let entries = get_cwd_content(".", true)?;

    let select = Dialog::around(
        SelectView::<String>::new()
            .on_submit(change_dir)
            .with_all_str(entries)
            .with_name("select")
            .scrollable()
            .fixed_size((30, 20)),
    )
    .title(get_pwd())
    .with_name("dialog");

    s.add_layer(select);

    Ok(())
}

fn initialize_events(s: &mut Cursive) {
    s.add_global_callback('q', |s| s.quit());
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
    let mut siv = Cursive::default();

    initialize_content_select(&mut siv)?;
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
