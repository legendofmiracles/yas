use sha2;
use sha2::Digest;
use std::fs;
use std::io::Write;
use users::{get_current_uid, get_user_by_uid};
struct Hash {
    format: u8,
    hash: String,
    salt: String,
}

pub fn get_hash() {
    let contents: String = fs::read_to_string("/etc/shadow").unwrap();
    // iterate over each line of the file and filter it by the username of the invoking user.
    let user = get_user_by_uid(get_current_uid()).unwrap();

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
    let pwd = ask_pass(user.name().to_str().unwrap());
    let hash = match hash_struct.format {
        1 => todo!(),
        2 => todo!(),
        3 => todo!(),
        4 => todo!(),
        5 => todo!(),
        6 => sha256(hash_struct),
        _ => panic!("unknown encryption method"),
    };
}

fn ask_pass(user: &str) -> String {
    let mut input = String::new();
    print!("yas: password for user {}: ", user);
    std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    return input;
}

fn sha256(hash_struct: Hash) -> String {
    let mut hasher = sha2::Sha256::new();
    let mut hash_with_salt = hash_struct.hash;
    println!("{}", hash_with_salt);
    hash_with_salt.push_str(&hash_struct.salt);
    println!("{}", hash_with_salt);
    hasher.update(&hash_with_salt);
    let final_res = hasher.finalize();
    println!("{:?}", final_res);
    return std::str::from_utf8(&final_res).unwrap().to_string();
}
