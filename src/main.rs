use std::os::unix::process::CommandExt;
use std::process::Command;
mod hash;
fn main() {
    hash::get_hash();
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    // if the command runs sucessfully, this program will immediately quit.
    // Otherwise the program will inform the user that it didn't start perfectly fine
    let command = Command::new(args.remove(0))
        .args(args)
        .exec()
        .raw_os_error();
    let error = std::io::Error::from_raw_os_error(command.unwrap());
    println!("yas: {}", error);
}
