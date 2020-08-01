#[macro_use]
extern crate clap;

use clap::{Shell, App, AppSettings};

fn main() {
    let yaml = load_yaml!("src/cli.yml");
    let mut app = App::from_yaml(yaml)
        .setting(AppSettings::ArgRequiredElseHelp);
    app.gen_completions("project-manager",
                        Shell::Bash,
                        "completions/");
    app.gen_completions("project-manager",
                        Shell::Zsh,
                        "completions/");
}

