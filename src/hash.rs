use hex;
use sha2::Digest;
use std::fs;
use std::io::Write;
use users::{get_current_uid, get_user_by_uid};
// #[derive(Copy)]
struct Hash {
    format: u8,
    hash: String,
    salt: String,
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

    for i in 0..3 {
        let pwd = ask_pass(user.name().to_str().unwrap());
        let is_match = match hash_struct.format {
            1 => todo!(),
            2 => todo!(),
            3 => todo!(),
            4 => todo!(),
            5 => todo!(),
            6 => sha512(&hash_struct, pwd),
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

fn sha512(hash_struct: &Hash, password: String) -> bool {
    let mut hasher = sha2::Sha512::new();
    let mut hash_with_salt = password;
    hash_with_salt.push_str(&hash_struct.salt);
    hasher.update(&hash_with_salt.as_bytes());
    let final_res = hasher.finalize();
    println!("{:?}", final_res);
    let encoded = hex::encode(final_res);
    println!("{:?}", encoded);
    let decoded = hex::decode(encoded).expect("error when decoding");
    println!("{:?}", std::str::from_utf8(&decoded));
    //if hash_struct.hash == std::str::from_utf8(&hex::decode(encoded).unwrap()).unwrap() {
    if true {
        true
    } else {
        false
    }
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
