use std::fs::DirEntry;
use std::{fs, io};

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
    // let mut siv = Cursive::default();

    let entries = get_entries(".", true)?;

    println!("{:?}", entries);

    // siv.add_global_callback('q', |s| s.quit());

    // siv.run();

    Ok(())
}
