use super::{ConfigurationError, InstallError};
use crate::{console::Console, error::Error};
use std::fmt::Display;

pub trait HostStep: 'static + Sized {
    type ConfigurationError: Error + Into<ConfigurationError>;
    type InstallError: Display + Into<InstallError>;

    fn configure(console: &mut Console) -> Result<Self, Self::ConfigurationError>;

    fn confirm(&self) -> Vec<(&str, String)>;

    fn install_message(&self) -> String;
    fn install(self) -> Result<(), Self::InstallError>;
}

macro_rules! steps {
    ($run_install_step: ident, [$($mod: ident ::$name: ident),*]) => {
        $(mod $mod;)*

        pub enum ConfigurationError {
            $($name(<$mod::$name as $crate::host::HostStep>::ConfigurationError)),*
        }

        pub enum InstallError {
            $($name(<$mod::$name as $crate::host::HostStep>::InstallError)),*
        }

        struct Configuration {
            $($mod: $mod::$name),*
        }

        impl Configuration {
            pub(self) fn get(console: &mut $crate::console::Console) -> Result<Self, $crate::host::ConfigurationError> {
                $(
                    let $mod = <$mod::$name as $crate::host::HostStep>::configure(console)?;
                )*

                Ok(Configuration {
                    $($mod),*
                })
            }

            pub(self) fn confirmation(&self) -> Vec<(&str, String)> {
                let mut values = Vec::new();

                $(
                    values.extend(self.$mod.confirm());
                )*

                values
            }

            pub(self) fn steps(&self) -> i32 {
                // This adds 1 for each item in the configuration.
                // The zero is required for the left side of the first "+"
                0 $( + {
                    let _ = self.$mod; // This is here to tell rust where to source the repitition
                    1
                })*
            }

            pub(self) fn install(self, window: &mut $crate::console::ProgressWindow)-> $crate::host::HostInstallResult<()> {
                $(
                    $run_install_step(window, self.$mod)?;
                )*

                Ok(())
            }
        }

        impl $crate::error::Error for ConfigurationError {}

        impl ::std::fmt::Display for ConfigurationError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(ConfigurationError::$name(error) => error.fmt(f)),*
                }
            }
        }

        $(
            impl From<<$mod::$name as $crate::host::HostStep>::ConfigurationError> for ConfigurationError {
                fn from(error: <$mod::$name as $crate::host::HostStep>::ConfigurationError) -> Self {
                    ConfigurationError::$name(error)
                }
            }
        )*

        impl ::std::fmt::Display for InstallError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(InstallError::$name(error) => error.fmt(f)),*
                }
            }
        }

        $(
            impl From<<$mod::$name as $crate::host::HostStep>::InstallError> for InstallError {
                fn from(error: <$mod::$name as $crate::host::HostStep>::InstallError) -> Self {
                    InstallError::$name(error)
                }
            }
        )*
    };
}
pub(super) use steps;
