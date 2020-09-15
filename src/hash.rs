use sha2::Digest;
use std::fs;

struct Hash {
    format: u8,
    hash: String,
    salt: String,
    password: String,
}

pub fn check_passwd(pwd: String, user: users::User) {
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
        password: pwd.replace("\n", ""),
    };
    let is_match = match hash_struct.format {
        1 => todo!(),
        2 => todo!(),
        3 => todo!(),
        4 => todo!(),
        5 => todo!(),
        6 => sha256(hash_struct),
        _ => panic!("unknown encryption method (╯°□°）╯︵ ┻━┻"),
    };
}

fn sha256(hash_struct: Hash) -> bool {
    let mut hasher = sha2::Sha256::new();
    let mut hash_with_salt = hash_struct.password;
    hash_with_salt.push_str(&hash_struct.salt);
    hasher.update(&hash_with_salt.as_bytes());
    let final_res = hasher.finalize();
    println!("{:?}", final_res);
    if hash_struct.hash == std::str::from_utf8(&final_res).unwrap() {
        true
    } else {
        false
    }
}
