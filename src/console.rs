use std::{
    fmt::Arguments,
    io::{stdin, stdout, StdinLock, StdoutLock, Write},
};

/// The input and output of the program
pub(crate) struct Console<'a> {
    output: StdoutLock<'a>,
    input: StdinLock<'a>,

    needs_newline: bool,
}

impl<'a> Console<'a> {
    /// Creates a new [`Console`]
    pub(crate) fn new() -> Self {
        let output = stdout().lock();
        let input = stdin().lock();

        Console {
            output,
            input,
            needs_newline: false,
        }
    }

    /// Prints `arguments` to stdout without a newline
    pub(crate) fn print(&mut self, arguments: Arguments) {
        self.needs_newline = true;
        self.output.write_fmt(arguments).unwrap();
    }

    /// Prints `arguments` to stdout with a newline
    pub(crate) fn println(&mut self, arguments: Arguments) {
        self.output.write_fmt(arguments).unwrap();
        self.output.write(b"\n").unwrap();
        self.needs_newline = false;
    }
}

impl<'a> Drop for Console<'a> {
    fn drop(&mut self) {
        if self.needs_newline {
            print!(self, "\n");
        }
    }
}

macro_rules! print {
    ($console: expr, $($arg:tt)*) => {
        $console.print(::std::format_args!($($arg)*))
    };
}

macro_rules! println {
    ($console: expr, $($arg:tt)*) => {
        $console.println(::std::format_args!($($arg)*))
    };
}

pub(crate) use {print, println};
