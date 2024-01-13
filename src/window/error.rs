/// An error the curses reported
pub struct CursesError;

impl std::error::Error for CursesError {}

impl std::fmt::Display for CursesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "curses reported an error")
    }
}

impl std::fmt::Debug for CursesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
