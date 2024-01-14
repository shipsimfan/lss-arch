use std::{
    ffi::OsStr,
    process::{Command as StdCommand, Stdio},
};

pub struct Command(StdCommand);

impl Command {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        let mut std_command = StdCommand::new(program);
        std_command
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .stdin(Stdio::null());

        Command(std_command)
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.0.args(args);
        self
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        if self.0.status()?.success() {
            Ok(())
        } else {
            Err(std::io::ErrorKind::Other.into())
        }
    }
}
