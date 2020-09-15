use std::os::unix::process::CommandExt;
use std::process::Command;
mod hash;
use std::io::Write;
use users::{get_current_uid, get_user_by_uid};
fn main() {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    hash::check_passwd(ask_pass(user.name().to_str().unwrap()), user);
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    // if the command runs sucessfully, this program will immediately quit.
    // Otherwise the program will inform the user that it didn't start perfectly fine
    let command = Command::new(args.remove(0))
        .args(args)
        .exec()
        .raw_os_error();
    let error = std::io::Error::from_raw_os_error(command.unwrap());
    println!("yas: {} (╯°□°）╯︵ ┻━┻", error);
}

fn ask_pass(user: &str) -> String {
    let mut input = String::new();
    print!("yas: password for user {}: ", user);
    std::io::stdout()
        .flush()
        .expect("yas: unable to flush stdout (╯°□°）╯︵ ┻━┻");
    std::io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input (╯°□°）╯︵ ┻━┻");
    input
}
