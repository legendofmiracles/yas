// use crate::hash;
// pub fn tui(hash: hash::Hash, user: users::User) -> bool {
//     let show_popup = move |s: &mut cursive::Cursive, password: &str| {
//         println!("{}", password);
//         let is_match = match hash.format {
//             1..=6 => crate::hash::sha512(&hash, password.to_string()),
//             _ => panic!("unknown encryption method (╯°□°）╯︵ ┻━┻"),
//         };
//         if is_match {
//             let mut args: Vec<String> = std::env::args().collect();
//             args.remove(1);
//             if args.len() == 0 || args[0] == "-h" || args[0] == "--help" {
//                 eprintln!("yas - execute commands as the root user\n\nusage: yas [-h/--help] <command> <arguments for the command, this can be chained infinite>");
//                 std::process::exit(1);
//             }
//             crate::do_the_actual_thing(args);
//         } // else if i != 3 {                               //
//           //     eprintln!("yas: wrong password. Nice try."); //
//           // }                                                //
//     };
//     use cursive::view::{Nameable, Resizable};
//     use cursive::views::{Dialog, EditView};
//     let mut siv = cursive::default();
//     siv.add_layer(
//         Dialog::new()
//             .title(format!(
//                 "Enter password for user {}",
//                 user.name().to_str().unwrap()
//             ))
//             // Padding is (left, right, top, bottom)
//             .padding_lrtb(1, 1, 1, 0)
//             .content(
//                 EditView::new()
//                     // Call `show_popup` when the user presses `Enter`
//                     .on_submit(show_popup)
//                     // Give the `EditView` a name so we can refer to it later.
//                     .with_name("pwd"),
//             )
//             .button("Ok", move |s| {
//                 // This will run the given closure, *ONLY* if a view with the
//                 // correct type and the given name is found.
//                 let name = s
//                     .call_on_name("pwd", |view: &mut EditView| {
//                         // We can return content from the closure!
//                         view.get_content()
//                     })
//                     .unwrap();

//                 // Run the next step
//                 show_popup(s, &name);
//             }),
//     );
//     siv.run();
//     return true;
// }
