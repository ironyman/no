// no - output a string repeatedly until killed
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn cmd_line() -> Option<String> {
    let mut res = std::env::args().skip(1)
        .fold("".to_string(), |accumulator, arg| accumulator + &arg + " ");
    if None == res.pop() {
        return None
    } else {
        return Some(res)
    }
}

fn exit_handler(should_exit: Arc<AtomicBool>) -> io::Result<()> {
    ctrlc::set_handler(move || {
        should_exit.store(true, Ordering::Relaxed);
    }).map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

enum UsagePrinted {
    Printed,
    Nope
}

fn setup_usage() -> io::Result<UsagePrinted> {
    if let Some(first) = std::env::args().nth(1) {
        if !first.starts_with("-") {
            return Ok(UsagePrinted::Nope)
        }

        let matches = clap::App::new("no")
            .version("0.1.0")
            .about("Repeatedly output a line with all specified STRING(s), or 'y'.")
            .get_matches_safe();

        if let Err(e) = matches {
            println!("{}", e);
        }

        return Ok(UsagePrinted::Printed)
    }
    Ok(UsagePrinted::Nope)
}

fn main() -> io::Result<()> {
    if let Ok(UsagePrinted::Printed) = setup_usage() {
        return Ok(())
    }

    let should_exit = Arc::new(AtomicBool::new(false));
    exit_handler(should_exit.clone())?;

    let buf = cmd_line().unwrap_or(String::from("y")) + "\n";
    let mut stdout = io::stdout();
    while !should_exit.load(Ordering::Relaxed) {
        stdout.write(buf.as_bytes())?;
    }

    Ok(())
}
