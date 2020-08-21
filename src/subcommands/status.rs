use clap::ArgMatches;
use crate::setup::SettingsFile;
use std::process::Stdio;

pub fn status(matches: ArgMatches, settings_file: SettingsFile) {
    let repo_name = matches
        .subcommand_matches("status").unwrap()
        .value_of("PROJ_NAME");

    match repo_name {
        Some(name) => {
            let repo_path = settings_file.get_repo_by_name(name)["path"].clone();
            println!("Status of {} in {}", name, repo_path);
            exec_git(
                vec![
                    "-C",
                    repo_path.as_str().unwrap(),
                    "status"
                ],
                Option::from(Stdio::inherit()),
                Option::from(Stdio::inherit())
            ).wait().unwrap();

        },
        None => {
            let repos = settings_file.list_repos();
            for member in repos.members() {
                let path = member["path"].as_str().unwrap();
                let name = member["name"].as_str().unwrap();
                let print_string = format!("Status of {} in {}", name, path);
                println!("{}\n", print_string);
                exec_git(
                    vec![
                        "-C",
                        path,
                        "status"
                    ],
                    Option::from(Stdio::inherit()),
                    Option::from(Stdio::inherit())
                ).wait().unwrap();
                println!("{}", "#".repeat(50));
            }
        }
    };

}
