use std::{path::PathBuf, process::Command};
use xtask::{cargo, run_or_panic};

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
        "man-pages" => cargo(["build", "--package", "fish-build-man-pages"]),
        "html-docs" => build_html_docs(command_args),
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

fn build_html_docs(args: &[String]) {
    let html_dir = match args {
        [html_dir] => html_dir,
        _ => panic!(
            "Expected the directory to write HTML docs to as the only argument, but instead got {args:?}"
        ),
    };

    let fish_indent_path = if let Some(fish_indent_path) =
        fish_build_helper::env_var("FISH_INDENT_FOR_BUILDING_DOCS")
    {
        PathBuf::from(fish_indent_path)
    } else {
        // Build fish_indent if no existing one is specified.
        cargo([
            "build",
            "--bin",
            "fish_indent",
            "--profile",
            "dev",
            "--no-default-features",
        ]);
        fish_build_helper::fish_build_dir()
            .join("debug")
            .join("fish_indent")
    };
    // Set path so `sphinx-build` can find `fish_indent`.
    // Create tempdir to store symlink to fish_indent.
    // This is done to avoid adding other binaries to the PATH.
    let tempdir = fish_tempfile::new_dir().unwrap();
    std::os::unix::fs::symlink(
        std::fs::canonicalize(fish_indent_path).unwrap(),
        tempdir.path().join("fish_indent"),
    )
    .unwrap();
    let new_path = format!(
        "{}:{}",
        tempdir.path().to_str().unwrap(),
        fish_build_helper::env_var("PATH").unwrap()
    );
    let doc_src_dir = fish_build_helper::workspace_root().join("doc_src");
    std::fs::create_dir_all(html_dir).unwrap();
    run_or_panic(Command::new("sphinx-build").env("PATH", new_path).args([
        "-j",
        "auto",
        "-q",
        "-b",
        "html",
        "-c",
        doc_src_dir.to_str().unwrap(),
        doc_src_dir.to_str().unwrap(),
        html_dir,
    ]))
}
