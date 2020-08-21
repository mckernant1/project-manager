use std::process::{Stdio, Child, Command};

fn exec_git(args: Vec<&str>, stdio: Option<Stdio>, stderr: Option<Stdio>) -> Child {
    let stdio_option = stdio.unwrap_or(Stdio::null());
    let stderr_option = stderr.unwrap_or(Stdio::null());

    Command::new("git")
        .args(args)
        .stdout(stdio_option)
        .stderr(stderr_option)
        .spawn().unwrap()
}
