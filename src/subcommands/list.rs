use clap::ArgMatches;
use crate::setup::SettingsFile;

pub fn list(matches: ArgMatches, settings_file: SettingsFile) {
    let repos = settings_file.list_repos();
    let plain_flag = matches.subcommand_matches("ls")
        .unwrap().is_present("plain");
    for member in repos.members() {
        if ! plain_flag {
            println!("{} -> {}",
                     member["name"].to_string(),
                     member["path"].to_string()
            );
        } else {
            println!("{}", member["name"].to_string());
        }
    }
}
