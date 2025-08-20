use std::process::Command;
use xtask::run_or_panic;

fn main() {
    // Args passed to xtasks, not including the binary name which gets set automatically.
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        panic!("Called xtask without arguments. Doing nothing.");
    }

    let command = &args[0];
    let command_args = &args[1..];
    match command.as_str() {
        "check" => run_checks(command_args),
        other => {
            panic!("Unknown xtask: {other}");
        }
    }
}

fn run_checks(args: &[String]) {
    if !args.is_empty() {
        panic!("Args passed to `check` when none were expected: {args:?}");
    }
    let repo_root_dir = fish_build_helper::workspace_root();
    let check_script = repo_root_dir.join("build_tools").join("check.sh");
    run_or_panic(&mut Command::new(check_script));
}
