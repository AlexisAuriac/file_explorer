use std::fs::Metadata;
use std::path::PathBuf;
use std::{fs, io};

use cursive::traits::*;
use cursive::views::{Dialog, EditView, SelectView};
use cursive::Cursive;

fn get_cwd_content(path: &str, hide: bool) -> Result<Vec<(String, Metadata)>, io::Error> {
    let entries = fs::read_dir(path)?.collect::<Result<Vec<_>, io::Error>>()?;

    let entries = if hide {
        entries
            .into_iter()
            .filter(|s| match s.file_name().to_str() {
                Some(s) if !s.starts_with('.') => true,
                _ => false,
            })
            .collect::<Vec<_>>()
    } else {
        entries
    };

    let mut entries = entries
        .iter()
        .map(|entry| {
            (
                entry.file_name().to_str().unwrap().to_string(),
                entry.metadata().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    entries.insert(
        0,
        (
            "..".to_string(),
            std::fs::File::open(PathBuf::from(".."))
                .unwrap()
                .metadata()
                .unwrap(),
        ),
    );
    entries.insert(
        0,
        (
            ".".to_string(),
            std::fs::File::open(PathBuf::from("."))
                .unwrap()
                .metadata()
                .unwrap(),
        ),
    );

    Ok(entries)
}

fn update_title(s: &mut Cursive) {
    let pwd = PathBuf::from(".");
    let pwd = fs::canonicalize(pwd).unwrap();
    let mut dialog = s.find_name::<Dialog>("dialog").unwrap();

    dialog.set_title(pwd.to_str().unwrap());
}

fn update_content(s: &mut Cursive) {
    let entries = get_cwd_content(".", true).unwrap();
    let entry_names = entries.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();

    select.clear();
    select.add_all_str(entry_names);
}

fn change_dir(s: &mut Cursive, name: &str) {
    let metadata = std::fs::File::open(PathBuf::from(name))
        .unwrap()
        .metadata()
        .unwrap();

    if !metadata.is_dir() {
        s.add_layer(
            Dialog::around(
                EditView::new()
                    .on_submit(|s, _| s.quit())
                    .with_name("edit_cmd"),
            )
            .title("Exec command"),
        );
        return;
    }

    std::env::set_current_dir(PathBuf::from(name)).unwrap();

    update_title(s);
    update_content(s);
}

fn exec_command(file: &str, cmd: &str) {
    let parts = cmd.split_whitespace().collect::<Vec<_>>();

    match &parts[..] {
        [] => eprintln!("Empty command"),
        [cmd] => {
            if let Err(err) = std::process::Command::new(cmd).arg(file).status() {
                eprintln!("{}: {}", cmd, err);
            }
        }
        _ => {
            let cmd = parts[0];
            let args = &parts[1..];

            if let Err(err) = std::process::Command::new(cmd)
                .args(args)
                .arg(file)
                .status()
            {
                eprintln!("{}: {}", cmd, err);
            }
        }
    }
}

fn main() -> io::Result<()> {
    let pwd = PathBuf::from(".");
    let pwd = fs::canonicalize(pwd)?;

    let mut siv = Cursive::default();

    let entries = get_cwd_content(".", true)?;

    let entry_names = entries.iter().map(|(name, _)| name).collect::<Vec<_>>();

    let select = Dialog::around(
        SelectView::<String>::new()
            .on_submit(change_dir)
            .with_all_str(entry_names)
            .with_name("select")
            .scrollable()
            .fixed_size((30, 20)),
    )
    .title(pwd.to_str().unwrap())
    .with_name("dialog");

    siv.add_layer(select);

    siv.add_global_callback('q', |s| s.quit());

    siv.run();

    let edit = siv.find_name::<EditView>("edit_cmd");

    if edit.is_none() {
        return Ok(());
    }

    let cmd = edit.unwrap().get_content().to_string();
    let file = siv
        .find_name::<SelectView<String>>("select")
        .unwrap()
        .selection()
        .unwrap()
        .to_string();

    drop(siv);

    exec_command(&file, &cmd);

    Ok(())
}
