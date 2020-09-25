mod hash;

#[cfg(feature = "tui")]
pub fn tui(hash: hash::Hash, user: users::User) -> bool {
    use cursive::views::{Dialog, EditView};
    return false;
}
