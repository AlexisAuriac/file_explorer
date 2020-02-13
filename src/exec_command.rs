use std::process::Command;

pub fn exec_command(file: &str, cmd: &str) {
    let parts = cmd.split_whitespace().collect::<Vec<_>>();

    match &parts[..] {
        [] => eprintln!("Empty command"),
        [cmd] => {
            if let Err(err) = Command::new(cmd).arg(file).status() {
                eprintln!("{}: {}", cmd, err);
            }
        }
        _ => {
            let cmd = parts[0];
            let args = &parts[1..];

            if let Err(err) = Command::new(cmd).args(args).arg(file).status() {
                eprintln!("{}: {}", cmd, err);
            }
        }
    }
}
