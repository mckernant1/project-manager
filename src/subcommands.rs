use clap::ArgMatches;
use json::JsonValue;
use std::process::Command;
use crate::setup::SettingsFile;

pub fn clone(matches: ArgMatches, settings_file: SettingsFile) {

    let repo_string = matches
        .subcommand_matches("clone").unwrap()
        .value_of("LINK").unwrap().to_string().clone();

    let json = settings_file.get_settings_json();
    let default_dir = json["defaultDir"].as_str().unwrap().clone();
    let repo_name: String = repo_string.split('/').last().unwrap()
        .chars().take_while(|c| { c != &'.'}).collect();
    let repo_dir = format!("{}/{}", default_dir, repo_name);
    println!("cloning {} to {}", repo_string, repo_dir);


    let thread = Command::new("git")
        .arg("clone")
        .arg(repo_string)
        .arg(repo_dir)
        .spawn().unwrap();
}

pub fn pull(matches: ArgMatches, settings_file: SettingsFile) {

}

pub fn list(matches: ArgMatches, settings_file: SettingsFile) {

}

pub fn add(matches: ArgMatches, settings_file: SettingsFile) {

}

pub fn delete(matches: ArgMatches, settings_file: SettingsFile) {

}
