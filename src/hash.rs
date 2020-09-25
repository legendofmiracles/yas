use crate::tui;
use std::fs;
use users::{get_current_uid, get_user_by_uid};
pub struct Hash {
    pub format: u8,
    pub hash: String,
    pub salt: String,
}

pub fn check_passwd() -> bool {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let contents: String = fs::read_to_string("/etc/shadow")
        .expect("yas: error when reading from /etc/shadow file (╯°□°）╯︵ ┻━┻");

    // iterate over each line of the file and filter it by the username of the invoking user.
    let items: Vec<&str> = contents
        .lines()
        .find(|x| x.contains(user.name().to_str().unwrap()))
        .unwrap()
        .split(":")
        .collect();
    let pre_hash = items[1];
    let hash_non_struct = pre_hash.split("$").collect::<Vec<&str>>();
    let hash_struct: Hash = Hash {
        format: hash_non_struct[1].parse().unwrap(),
        hash: hash_non_struct[3].to_string(),
        salt: hash_non_struct[2].to_string(),
    };
    if cfg!(feature = "tui") {
        return tui::tui(hash_struct, user);
    } else {
        return no_tui(hash_struct, user);
    }
}

pub fn sha512(hash_struct: &Hash, password: String) -> bool {
    let shadow = format!(
        "${}${}${}",
        hash_struct.format, hash_struct.salt, hash_struct.hash
    );
    return pwhash::unix::verify(password, &shadow);
}

fn ask_pass(user: &str) -> String {
    let pass = rpassword::prompt_password_stderr(&format!(
        "{}yas: password for user {}: ",
        String::from_utf8(vec![7]).unwrap_or_default(),
        user
    ))
    .unwrap();
    return pass;
}

fn no_tui(hash_struct: Hash, user: users::User) -> bool {
    for i in 0..3 {
        let pwd = ask_pass(user.name().to_str().unwrap());
        let is_match = match hash_struct.format {
            1..=6 => sha512(&hash_struct, pwd),
            _ => panic!("unknown encryption method (╯°□°）╯︵ ┻━┻"),
        };
        if is_match {
            return true;
        } else if i != 3 {
            eprintln!("yas: wrong password. Nice try.");
        }
    }
    eprintln!("yas: three wrong passwords. Quitting...");
    return false;
}
