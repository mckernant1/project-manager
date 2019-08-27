use clap::ArgMatches;
use json::JsonValue;
use std::process::{Command, Stdio};
use crate::setup::SettingsFile;

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

    let mut child = Command::new("git")
        .arg("clone")
        .arg(repo_string)
        .arg(repo_dir.clone())
        .spawn().unwrap();
    child.wait().unwrap();
    settings_file.clone().add_repo(
        repo_dir.clone().as_str(),
        repo_name.clone().as_str(),
    );
}

pub fn pull(matches: ArgMatches, settings_file: SettingsFile) {
    let repos = settings_file.list_repos();

    if matches.subcommand_matches("pull").unwrap()
        .is_present("NAME") {
        let repo_name = matches
            .subcommand_matches("pull").unwrap()
            .value_of("NAME").unwrap();
        let path = repos[repo_name]["path"].to_string();
        println!("{}; {}", repo_name, path);
        Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("pull")
            .spawn().unwrap()
            .wait().unwrap();
    } else {
        let mut child_list: Vec<_> = vec![];
        for member in repos.members() {
            println!("Pulling: {}", member["name"].to_string());
            child_list.push(
                Command::new("git")
                    .arg("-C")
                    .arg(member["path"].to_string())
                    .arg("pull")
                    .stdout(Stdio::null())
                    .spawn().unwrap()
            )
        }
        for mut child in child_list {
           child.wait().unwrap();
        }
    }
    println!();
}

pub fn list(matches: ArgMatches, settings_file: SettingsFile) {
    let repos = settings_file.list_repos();
    for member in repos.members() {
        println!("{}", member["name"].to_string());
    }
}

pub fn add(matches: ArgMatches, settings_file: SettingsFile) {}

pub fn delete(matches: ArgMatches, settings_file: SettingsFile) {}
