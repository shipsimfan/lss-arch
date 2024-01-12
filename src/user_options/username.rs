use crate::{prompt, Console};

/// Initial user details
#[derive(Clone)]
pub(crate) struct Username(String);

/// The default username for the initial user
const DEFAULT_USERNAME: &str = "lhart";

impl Username {
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

        Username(username)
    }
}

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
