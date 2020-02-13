use std::fs::DirEntry;
use std::path::PathBuf;
use std::{fs, io};

use cursive::traits::*;
use cursive::views::{Dialog, SelectView};
use cursive::Cursive;

fn get_entries(path: &str, hide: bool) -> Result<Vec<DirEntry>, io::Error> {
    let entries = fs::read_dir(path)?.collect::<Result<Vec<_>, io::Error>>()?;

    if hide {
        Ok(entries
            .into_iter()
            .filter(|s| match s.file_name().to_str() {
                Some(s) if !s.starts_with('.') => true,
                _ => false,
            })
            .collect::<Vec<_>>())
    } else {
        Ok(entries)
    }
}

fn main() -> io::Result<()> {
    let pwd = PathBuf::from(".");
    let pwd = fs::canonicalize(pwd)?;

    let mut siv = Cursive::default();

    let entries = get_entries(".", false)?;

    let entry_names = entries
        .iter()
        .map(|file| file.file_name().to_str().unwrap().to_string())
        .collect::<Vec<_>>();

    println!("{:?}", entries);
    println!("{:?}", entry_names);

    let select = Dialog::around(
        SelectView::<String>::new()
            // .item_str("world")
            .with_all_str(entry_names)
            .with_name("select")
            .fixed_size((30, 20)),
    )
    .title(pwd.to_str().unwrap());

    siv.add_layer(select);

    siv.add_global_callback('q', |s| s.quit());

    siv.run();

    Ok(())
}
