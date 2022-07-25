pub struct User {
    pub(crate) active: bool,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) sign_in_count: u64,
}

pub fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

