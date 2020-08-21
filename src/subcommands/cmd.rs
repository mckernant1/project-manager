use clap::ArgMatches;
use crate::setup::SettingsFile;
use std::process::{Command, Stdio};
use json::JsonValue;
use std::io::{stdin, stdout, Write};

pub fn cmd(matches: ArgMatches, settings_file: SettingsFile) {
    let subcommand_matches = matches
        .subcommand_matches("cmd").unwrap();

    let repo_name = subcommand_matches
        .value_of("PROJ_NAME").unwrap();
    let cmd_name = subcommand_matches
        .value_of("CMD_NAME").unwrap();
    let display_output = !subcommand_matches
        .is_present("disable-output");

    let repo_json = settings_file.clone().get_repo_by_name(repo_name);

    let cmd_to_run = repo_json["cmds"][cmd_name].clone().to_string();

    if cmd_to_run == "null".to_string() {
        create_new_cmd(settings_file, cmd_name, repo_json.clone());
    } else {
        println!("Executing: '{}' in '{}'", cmd_to_run, repo_json["path"].as_str().unwrap());
        let cmd_split: Vec<&str> = cmd_to_run
            .split(" ")
            .collect::<Vec<&str>>();
        let cmd = cmd_split[0];
        let cmd_args = cmd_split.into_iter().skip(1).collect::<Vec<&str>>();
        Command::new(cmd)
            .current_dir(repo_json["path"].as_str().unwrap())
            .args(cmd_args)
            .stdout(
                if display_output { Stdio::inherit() } else { Stdio::null() }
            )
            .stderr(
                if display_output { Stdio::inherit() } else { Stdio::null() }
            )
            .spawn().unwrap()
            .wait().unwrap();
    }
}

pub fn cmds(matches: ArgMatches, settings_file: SettingsFile) {
    let repo_name = matches
        .subcommand_matches("cmds").unwrap()
        .value_of("PROJ_NAME").unwrap();

    let repo_json = settings_file.get_repo_by_name(repo_name);
    let title_string = format!("Commands for {}", repo_json["name"].as_str().unwrap());
    println!("{}\n{}", title_string.to_uppercase(),
             "#".repeat(title_string.len()));

    for member in repo_json["cmds"].entries() {
        let (name, cmd) = member;
        println!("name: {}\ncommand: {}\n", name, cmd.as_str().unwrap())
    }
}

fn create_new_cmd(settings_file: SettingsFile, cmd_name: &str, repo_json: JsonValue) {
    println!("This command does not exist.");
    print!("Please enter the command you want to run in this directory: ");
    stdout().flush().unwrap();
    let mut cmd_string = String::new();
    stdin().read_line(&mut cmd_string).unwrap();

    let mut repo_json_mut = repo_json.clone();
    repo_json_mut["cmds"][cmd_name] = cmd_string.replace("\n", "").into();

    settings_file.add_repo(repo_json_mut);
}
