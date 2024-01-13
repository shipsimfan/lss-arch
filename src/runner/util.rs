/// Attempts to run a step
macro_rules! try_step {
    ($step: expr, |$error: ident| $failure: expr) => {
        use $crate::error::Error;

        match $step {
            Ok(result) => result,
            Err($error) => match $error.is_curses_error() {
                true => return Err($error.into_curses_error().unwrap()),
                false => {
                    $failure;
                    return Ok(false);
                }
            },
        }
    };
}

pub(super) use try_step;
