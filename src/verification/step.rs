use std::fmt::Display;

pub trait VerificationStep {
    type Error: Display + Into<NotVerifiedError>;

    const MESSAGE: &'static str;

    fn execute() -> Result<(), Self::Error>;
}

macro_rules! verification_steps {
    ($run_step: ident, [$($mod: ident ::$name: ident),*]) => {
        $(mod $mod;)*

        fn run_steps(window: &mut $crate::console::ProgressWindow) -> $crate::verification::VerificationResult<()> {
            $(
                $run_step::<$mod::$name>(window)?;
            )*

            Ok(())
        }

        pub enum NotVerifiedError {
            $($name(<$mod::$name as $crate::verification::step::VerificationStep>::Error)),*
        }

        impl ::std::fmt::Display for NotVerifiedError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(NotVerifiedError::$name(error) => error.fmt(f)),*
                }
            }
        }

        $(
            impl From<<$mod::$name as $crate::verification::step::VerificationStep>::Error> for NotVerifiedError {
                fn from(error: <$mod::$name as $crate::verification::step::VerificationStep>::Error) -> Self {
                    NotVerifiedError::$name(error)
                }
            }
        )*
    };
}

pub(super) use verification_steps;

use super::NotVerifiedError;
