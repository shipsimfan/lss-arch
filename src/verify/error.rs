/// An error while verifying the program state
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum VerifyError {
    NotRunningAsRoot,
    NotUEFI64Bit,
    NotConnectedToInternet,
}

impl std::error::Error for VerifyError {}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyError::NotRunningAsRoot => {
                write!(f, "currently not running as root, please run this as root")
            }
            VerifyError::NotUEFI64Bit => {
                write!(f, "the system was not booted in 64-bit mode with UEFI")
            }
            VerifyError::NotConnectedToInternet => write!(
                f,
                "cannot ping archlinux.org, verify you are connected to the internet and try again"
            ),
        }
    }
}

impl std::fmt::Debug for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
