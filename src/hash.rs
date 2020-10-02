use std::fs;
struct Hash {
    format: u8,
    hash: String,
    salt: String,
}

pub fn check_passwd(args: &Vec<String>, user: String) -> bool {
    let contents: String = fs::read_to_string("/etc/shadow")
        .expect("yas: error when reading from /etc/shadow file (╯°□°）╯︵ ┻━┻");

    // iterate over each line of the file and filter it by the username of the invoking user.
    let items: Vec<&str> = contents
        .lines()
        .find(|x| x.contains(&user))
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
    #[cfg(feature = "tui")]
    return tui(hash_struct, user, args.to_vec());
    #[cfg(not(feature = "tui"))]
    return no_tui(hash_struct, user);
}

fn decode(hash_struct: &Hash, password: String) -> bool {
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

fn no_tui(hash_struct: Hash, user: String) -> bool {
    for i in 0..3 {
        let pwd = ask_pass(&user);
        let is_match = match hash_struct.format {
            1..=6 => decode(&hash_struct, pwd),
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
#[cfg(feature = "tui")]
fn tui(hash_struct: Hash, user: String, args: Vec<String>) -> bool {
    use cursive::views::{Dialog, EditView, TextView};
    let mut siv = cursive::default();
    siv.add_layer(
        Dialog::new()
            .title(format!("Enter password for user {}", user))
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new().secret().on_submit(
                    move |s: &mut cursive::Cursive, password: &str| {
                        let is_match = match hash_struct.format {
                            1..=6 => decode(&hash_struct, password.to_string()),
                            _ => panic!("unknown encryption method (╯°□°）╯︵ ┻━┻"),
                        };
                        if is_match {
                            s.set_user_data(true);
                            s.quit();
                        } else {
                            s.add_layer(
                                Dialog::new()
                                    .title("Wrong password")
                                    .content(TextView::new("Wrong password. Nice try"))
                                    .button("retry", |s: &mut cursive::Cursive| -> _ {
                                        s.pop_layer();
                                    }),
                            );
                        }
                    },
                ), /////////////////////////////////////////////////////////////////////////
                   // .on_edit_mut(|s: &cursive::Cursive, password: &mut str, _: usize| { //
                   //     password = "LOLSU"                                              //
                   // }),                                                                 //
                   /////////////////////////////////////////////////////////////////////////
            ),
    );
    siv.run();
    if *siv.user_data().unwrap() {
        std::mem::drop(siv);
        // This function quits the program and doesn't return control
        // And i can't return a bool, because
        // 1. i would have to add a new mutable argument, which i can't because the lib only wants 2 args
        // 2. I can't return a boolean to the .submit function
        // Thats why we have to call the function on our own
        crate::do_the_actual_thing(args.to_vec());
    }
    // This only runs, if the above bool is false, as the code it calls in the statements, it definitly quits the program
    return false;
}
