use std::os::unix::process::CommandExt;
use std::process::Command;
pub mod hash;
pub mod tui;
fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    // returns the first argument, in this case which binary was ran
    args.remove(0);
    if args.len() == 0 || args[0] == "-h" || args[0] == "--help" {
        eprintln!("yas - execute commands as the root user\n\nusage: yas [-h/--help] [-v/--version] <command> <arguments for the command, this can be chained infinite>");
        std::process::exit(1);
    } else if args.len() == 0 && args[0] == "-v" || args[0] == "--version" {
        eprintln!("yas 0.1.0");
        std::process::exit(1);
    }
    let matches: bool = hash::check_passwd();
    if matches {
        // this function will either immediately quit the program (as seen in the comments below), or it will inform the user of a error and then quit,
        // thus making returning a error incredibly stupid
        do_the_actual_thing(args);
    } else {
        // We exit here, because the bool only gets returned as false, if we had three wrong password inputs.
        std::process::exit(1);
    }
}

pub fn do_the_actual_thing(mut args: Vec<String>) {
    // if the command runs sucessfully, this program will immediately quit.
    // Otherwise the program will inform the user that it didn't start perfectly fine
    let command = Command::new(args.remove(0))
        .args(args)
        .env("HOME", std::env::var("HOME").unwrap_or_default())
        .uid(0)
        .exec()
        .raw_os_error();
    let error = std::io::Error::from_raw_os_error(command.unwrap());
    println!("yas: {} (╯°□°）╯︵ ┻━┻", error);
    std::process::exit(1)
}
