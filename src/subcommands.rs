use clap::ArgMatches;
use json::JsonValue;
use std::process::{Command, Stdio};
use crate::setup::SettingsFile;
use std::path::{PathBuf, Path};
use std::fs;
use std::ffi::OsStr;

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
        let path = repos.members().find(|m| {
            m["name"].to_string() == repo_name
        }).unwrap()["path"].to_string();
        println!("Pulling: {}", repo_name);
        Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("pull")
            .spawn().unwrap()
            .wait().unwrap();
        println!("Done pulling: {}", repo_name)
    } else {
        let mut child_list: Vec<_> = vec![];
        for member in repos.members() {
            println!("Pulling: {}", member["name"].clone().to_string());
            child_list.push((
                member["name"].to_string(),
                Command::new("git")
                    .arg("-C")
                    .arg(member["path"].to_string())
                    .arg("pull")
                    .stdout(Stdio::null())
                    .spawn().unwrap()
            ))
        }
        for child_pair in child_list {
            let (child_name, mut child) = child_pair;
            child.wait().unwrap();
            println!("Done pulling: {}", child_name)
        }
    }
}

pub fn list(matches: ArgMatches, settings_file: SettingsFile) {
    let repos = settings_file.list_repos();
    for member in repos.members() {
        println!("{}", member["name"].to_string());
    }
}

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
            .take_while(|c| { c != &'/' } )
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();

    settings_file.add_repo(
        path_string,
        repo_name.as_str()
    );
}

pub fn delete(matches: ArgMatches, settings_file: SettingsFile) {

}
