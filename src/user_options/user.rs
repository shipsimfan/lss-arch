use crate::{prompt, Console};

/// Initial user details
#[derive(Clone)]
pub(crate) struct User {
    username: String,
}

const DEFAULT_USERNAME: &str = "lhart";

impl User {
    /// Gets the initial user details from the user
    pub(super) fn get(console: &mut Console) -> Self {
        prompt!(
            console,
            DEFAULT_USERNAME,
            "Enter a username for the initial user"
        );
        let username = console.readln();
        let username = if username.len() == 0 {
            DEFAULT_USERNAME.to_owned()
        } else {
            username
        };

        User { username }
    }

    pub(crate) fn username(&self) -> &str {
        &self.username
    }
}
