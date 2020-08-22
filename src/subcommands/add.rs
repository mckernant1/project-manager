use clap::ArgMatches;
use crate::setup::SettingsFile;

use std::fs;
use std::path::PathBuf;

pub fn add(matches: ArgMatches, settings_file: SettingsFile) {
    let path_string =
        matches.subcommand_matches("add").unwrap()
            .value_of("PATH").unwrap();
    let canonical_path =
        fs::canonicalize(
            PathBuf::from(path_string)
        ).unwrap();

    let path_string = canonical_path.as_os_str().to_str().unwrap();
    let repo_name =
        path_string.chars()
            .rev()
            .take_while(|c| { c != &'/' })
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();

    settings_file.add_repo(object! {
        "path" => path_string,
        "name" => repo_name.as_str(),
    });
}
