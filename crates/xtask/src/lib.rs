use std::{ffi::OsStr, process::Command};

pub fn run_or_panic(command: &mut Command) {
    match command.status() {
        Ok(exit_status) => {
            if !exit_status.success() {
                panic!(
                    "Command did not run successfully: {:?}",
                    command.get_program()
                )
            }
        }
        Err(err) => {
            panic!("Failed to run command: {err}");
        }
    }
}

pub fn cargo<I, S>(cargo_args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_or_panic(Command::new(env!("CARGO")).args(cargo_args));
}
