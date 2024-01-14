/// Attempts to run a step
macro_rules! try_step {
    ($step: expr, |$error: ident| $failure: expr) => {
        match $step {
            Ok(result) => result,
            Err($error) => match $crate::error::Error::is_curses_error(&$error) {
                true => return Err($crate::error::Error::into_curses_error($error).unwrap()),
                false => {
                    $failure;
                    return Ok(false);
                }
            },
        }
    };
}

pub(super) use try_step;
