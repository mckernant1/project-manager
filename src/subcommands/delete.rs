use clap::ArgMatches;
use crate::setup::SettingsFile;
use std::fs;

pub fn delete(matches: ArgMatches, settings_file: SettingsFile) {
    let subcommand_matches = matches
        .subcommand_matches("rm").unwrap();
    let repo_name = subcommand_matches
        .value_of("PROJ_NAME").unwrap();

    if subcommand_matches
        .is_present("remove-dir") {
        let repo = settings_file.clone().get_repo_by_name(repo_name);
        let repo_path = repo["path"].as_str().unwrap();
        fs::remove_dir_all(repo_path).unwrap();
    }

    settings_file.clone().delete_repo(repo_name)
}
