#[macro_use]
extern crate clap;
#[macro_use]
extern crate json;

mod subcommands;
mod setup;


use clap::{App, AppSettings, ArgMatches};
use setup::SettingsFile;
use crate::subcommands::clone::clone;
use crate::subcommands::pull::pull;
use crate::subcommands::list::list;
use crate::subcommands::add::add;
use crate::subcommands::delete::delete;
use crate::subcommands::status::status;


fn main() {
    let settings_file = SettingsFile::new();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();
    call_subcommands(matches, settings_file)
}

fn call_subcommands(matches: ArgMatches, settings_file: SettingsFile) {
    if matches.is_present("clone") {
        clone(matches, settings_file)
    } else if matches.is_present("pull") {
        pull(matches, settings_file)
    } else if matches.is_present("ls") {
        list(matches, settings_file)
    } else if matches.is_present("add") {
        add(matches, settings_file)
    } else if matches.is_present("rm") {
        delete(matches, settings_file)
    } else if matches.is_present("status") {
        status(matches, settings_file)
    }
}

