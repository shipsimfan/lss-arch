/// An error while verifying the program state
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum VerifyError {
    /// The program is not running as root
    NotRunningAsRoot,

    /// The OS was not booted by UEFI into 64-bit mode
    NotUEFI64Bit,

    /// The machine is not connected to the internet
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
