use std::os::unix::process::CommandExt;
use std::process::Command;
mod hash;
fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
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

fn do_the_actual_thing(mut args: Vec<String>) {
    // if the command runs sucessfully, this program will immediately quit.
    // Otherwise the program will inform the user that it didn't start perfectly fine
    let command = Command::new(args.remove(0))
        .args(args)
        .exec()
        .raw_os_error();
    let error = std::io::Error::from_raw_os_error(command.unwrap());
    println!("yas: {} (╯°□°）╯︵ ┻━┻", error);
}
