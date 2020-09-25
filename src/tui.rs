use crate::hash;
#[cfg(feature = "tui")]
pub fn tui(hash: hash::Hash, user: users::User) -> bool {
    use cursive::view::{Nameable, Resizable};
    use cursive::views::{Dialog, EditView};
    let mut siv = cursive::default();
    siv.add_layer(
        Dialog::new()
            .title(format!(
                "Enter password for user {}",
                user.name().to_str().unwrap()
            ))
            // Padding is (left, right, top, bottom)
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    // Call `show_popup` when the user presses `Enter`
                    .on_submit(show_popup)
                    // Give the `EditView` a name so we can refer to it later.
                    .with_name("pwd"),
            )
            .button("Ok", |s| {
                // This will run the given closure, *ONLY* if a view with the
                // correct type and the given name is found.
                let name = s
                    .call_on_name("pwd", |view: &mut EditView| {
                        // We can return content from the closure!
                        view.get_content()
                    })
                    .unwrap();

                // Run the next step
                show_popup(s, &name);
            }),
    );
    siv.run();
    return true;
    fn show_popup(s: &mut cursive::Cursive, password: &str) {
        println!("{}", password);
        let is_match = match hash.format {
            1..=6 => crate::hash::sha512(&hash, password.to_string()),
            _ => panic!("unknown encryption method (╯°□°）╯︵ ┻━┻"),
        };
        if is_match {
            crate::do_the_actual_thing(std::env::vars().collect::<Vec<String>>());
        } // else if i != 3 {                               //
          //     eprintln!("yas: wrong password. Nice try."); //
          // }                                                //
    }
}
