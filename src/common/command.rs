use std::{
    ffi::OsStr,
    process::{Command as StdCommand, Output, Stdio},
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

    pub fn stdout_piped(&mut self) -> &mut Command {
        self.0.stdout(Stdio::piped());
        self
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        self.0.arg(arg);
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.0.args(args);
        self
    }

    pub fn output(&mut self) -> std::io::Result<Output> {
        let result = self.0.output()?;
        if result.status.success() {
            Ok(result)
        } else {
            Err(std::io::ErrorKind::Other.into())
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        if self.0.status()?.success() {
            Ok(())
        } else {
            Err(std::io::ErrorKind::Other.into())
        }
    }
}
