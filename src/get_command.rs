use cursive::traits::*;

use cursive::views::{Dialog, EditView};
use cursive::Cursive;

fn submit_command(s: &mut Cursive, cmd: &str) {
    if cmd.trim() != "" {
        s.quit();
    } else {
        s.pop_layer();
        s.add_layer(Dialog::info("Empty command"));
    }
}

pub fn get_command(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(submit_command)
                .with_name("edit_cmd"),
        )
        .title("Exec command")
        .button("Cancel", |s| {
            s.pop_layer();
        })
        .button("Ok", |s| {
            let name = s
                .call_on_name("name", |view: &mut EditView| view.get_content())
                .unwrap();
            submit_command(s, &name);
        }),
    );
}
