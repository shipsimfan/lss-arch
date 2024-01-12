/// The user input was not confirmed
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct NotConfirmedError;

impl std::error::Error for NotConfirmedError {}

impl std::fmt::Display for NotConfirmedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the user did not confirm the install options, aborting")
    }
}

impl std::fmt::Debug for NotConfirmedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
