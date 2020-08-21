use clap::ArgMatches;
use crate::setup::SettingsFile;
use spinners::{Spinner, Spinners};

pub fn pull(matches: ArgMatches, settings_file: SettingsFile) {
    let repos = settings_file.list_repos();
    let subcommand_matches = matches.subcommand_matches("pull").unwrap();

    if subcommand_matches.is_present("PROJ_NAME") {
        let repo_name = subcommand_matches
            .value_of("NAME").unwrap();
        let path = repos.members().find(|m| {
            m["name"].to_string() == repo_name
        }).unwrap()["path"].to_string();
        println!("Pulling: {}", repo_name);
        exec_git(vec!["-C", path.as_str(), "pull"], Option::None, Option::None)
            .wait().unwrap();
        println!("Done pulling: {}", repo_name)
    } else {
        let mut child_list: Vec<_> = vec![];
        let spinner = Spinner::new(Spinners::Dots, "Pulling all repos...".into());
        for member in repos.members() {
            child_list.push(
                (member["path"].as_str().unwrap(),
                 exec_git(vec!["-C", member["path"].as_str().unwrap(), "pull"], Option::None, Option::None)
                )
            )
        }
        for (path, mut child) in child_list {
            let exit_status = child.wait().unwrap();
            if !exit_status.success() {
                eprintln!("\n{} has failed with status {}", path, exit_status.code().unwrap());
            }
        }
        spinner.stop();
        println!("\nDone!")
    }
}
