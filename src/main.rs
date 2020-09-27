use std::os::unix::process::CommandExt;
use std::process::Command;
pub mod hash;
pub mod tui;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use users::{get_current_uid, get_user_by_uid};

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
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let path = std::path::PathBuf::from(format!("/var/db/yas/{}", &user.name().to_str().unwrap()));
    let mut requires: bool = true;
    if path.exists() {
        let meta = fs::metadata(path).unwrap();
        let date = meta.created().unwrap();
        // as long as 5 minutes haven't passed yet, yas doesn't require a passsword
        // And also checking a ton of cases, that could mean that the file is has been tampered with
        if date.elapsed().unwrap() < std::time::Duration::new(300, 0)
            && meta.modified().unwrap().elapsed().unwrap() != date.elapsed().unwrap()
            && meta.permissions().mode() == 33152
        {
            requires = false;
        }
    }
    let matches: bool;
    if requires {
        matches = hash::check_passwd(&args, user.name().to_str().unwrap().to_string());
    } else {
        matches = true;
    }
    if matches {
        // this function will either immediately quit the program (as seen in the comments below), or it will inform the user of a error and then quit,
        // thus making returning a error incredibly stupid
        do_the_actual_thing(args, user.name().to_str().unwrap().to_string());
    } else {
        // We exit here, because the bool only gets returned as false, if we had three wrong password inputs.
        std::process::exit(1);
    }
}

pub fn do_the_actual_thing(mut args: Vec<String>, user: String) {
    if cache(user).is_err() {
        eprintln!("failed to create cache");
    }
    // if the command runs sucessfully, this program will immediately quit.
    // Otherwise the program will inform the user that it didn't start perfectly fine
    let command = Command::new(args.remove(0))
        .args(args)
        // .env("HOME", std::env::var("HOME").unwrap_or_default())
        .uid(0)
        .exec()
        .raw_os_error();
    let error = std::io::Error::from_raw_os_error(command.unwrap());
    println!("yas: {} (╯°□°）╯︵ ┻━┻", error);
    std::process::exit(1)
}

fn cache(user: String) -> std::io::Result<()> {
    fs::create_dir_all("/var/db/yas")?;
    let mut perms = fs::metadata("/var/db/yas")?.permissions();
    perms.set_mode(600);
    std::fs::set_permissions("/var/db/yas", perms)?;
    fs::remove_file(format!("/var/db/yas/{}", user)).unwrap_or_default();
    let f = fs::File::create(format!("/var/db/yas/{}", user))?;
    let mut perms = f.metadata()?.permissions();
    perms.set_mode(0o600);
    std::fs::set_permissions(format!("/var/db/yas/{}", user), perms)?;
    Ok(())
}
