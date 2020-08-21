use clap::ArgMatches;
use crate::setup::SettingsFile;
use std::process::Stdio;

pub fn clone(matches: ArgMatches, settings_file: SettingsFile) {
    let repo_string = matches
        .subcommand_matches("clone").unwrap()
        .value_of("LINK").unwrap().to_string().clone();

    let json_clone = settings_file
        .clone().get_settings_json().clone();
    let default_dir = json_clone["defaultDir"].as_str().unwrap().clone();
    let repo_name: String = repo_string.split('/').last().unwrap()
        .chars().take_while(|c| { c != &'.' }).collect();
    let repo_dir = format!("{}/{}", default_dir, repo_name);
    println!("Cloning {} to {}\n", repo_string, repo_dir);

    let mut child = exec_git(
        vec!["clone", repo_string.as_str(), repo_dir.clone().as_str()],
        Option::from(Stdio::inherit()), Option::from(Stdio::inherit())
    );
    child.wait().unwrap();
    settings_file.clone().add_repo(object! {
        "path" => repo_dir.clone().as_str(),
        "name" => repo_name.clone().as_str(),
    });
}
