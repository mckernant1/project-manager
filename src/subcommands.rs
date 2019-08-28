use clap::ArgMatches;
use std::process::{Command, Stdio, Child};
use crate::setup::SettingsFile;
use std::path::PathBuf;
use std::fs;
use std::io::{stdout, Write, stdin};
use json::JsonValue;

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
        Option::from(Stdio::piped()),
    );
    child.wait().unwrap();
    settings_file.clone().add_repo(object! {
        "path" => repo_dir.clone().as_str(),
        "name" => repo_name.clone().as_str(),
    });
}

pub fn pull(matches: ArgMatches, settings_file: SettingsFile) {
    let repos = settings_file.list_repos();

    if matches.subcommand_matches("pull").unwrap()
        .is_present("PROJ_NAME") {
        let repo_name = matches
            .subcommand_matches("pull").unwrap()
            .value_of("NAME").unwrap();
        let path = repos.members().find(|m| {
            m["name"].to_string() == repo_name
        }).unwrap()["path"].to_string();
        println!("Pulling: {}", repo_name);
        exec_git(vec!["-C", path.as_str(), "pull"], Option::None)
            .wait().unwrap();
        println!("Done pulling: {}", repo_name)
    } else {
        let mut child_list: Vec<_> = vec![];
        for member in repos.members() {
            println!("Pulling: {}", member["name"].clone().to_string());
            child_list.push((
                member["name"].to_string(),
                exec_git(vec!["-C", member["path"].as_str().unwrap(), "pull"], Option::None)
            ))
        }
        for child_pair in child_list {
            let (child_name, mut child) = child_pair;
            child.wait().unwrap();
            println!("Done pulling: {}", child_name)
        }
    }
}

fn exec_git(args: Vec<&str>, io: Option<Stdio>) -> Child {
    let io_option = match io {
        Some(io) => io,
        None => Stdio::null()
    };

    Command::new("git")
        .args(args)
        .stdout(io_option)
        .spawn().unwrap()
}

pub fn list(matches: ArgMatches, settings_file: SettingsFile) {
    let repos = settings_file.list_repos();
    let dirs_flag = matches.subcommand_matches("list")
        .unwrap().is_present("list-dirs");
    for member in repos.members() {
        if dirs_flag {
            println!("name: {}\npath: {}\n", member["name"].to_string(), member["path"].to_string());
        } else {
            println!("{}", member["name"].to_string());
        }
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

pub fn delete(matches: ArgMatches, settings_file: SettingsFile) {
    let repo_name = matches
        .subcommand_matches("rm").unwrap()
        .value_of("PROJ_NAME").unwrap();

    if matches.subcommand_matches("rm").unwrap()
        .is_present("remove-dir") {
        let repo = settings_file.clone().get_repo_by_name(repo_name);
        let repo_path = repo["path"].as_str().unwrap();
        fs::remove_dir_all(repo_path).unwrap();
    }

    settings_file.clone().delete_repo(repo_name)
}

pub fn cmd(matches: ArgMatches, settings_file: SettingsFile) {
    let repo_name = matches
        .subcommand_matches("cmd").unwrap()
        .value_of("PROJ_NAME").unwrap();
    let cmd_name = matches
        .subcommand_matches("cmd").unwrap()
        .value_of("CMD_NAME").unwrap();

    let repo_json = settings_file.clone().get_repo_by_name(repo_name);

    let cmd_to_run = repo_json["cmds"][cmd_name].clone().to_string();

    println!("Executing: '{}' in '{}'", cmd_to_run, repo_json["path"].as_str().unwrap());

    if cmd_to_run == "null".to_string() {
        create_new_cmd(settings_file, cmd_name, repo_json.clone());
    } else {
        let cmd_split: Vec<&str> = cmd_to_run
            .split(" ")
            .collect::<Vec<&str>>();
        let cmd = cmd_split[0];
        let cmd_args = cmd_split.into_iter().skip(1).collect::<Vec<&str>>();
        Command::new(cmd)
            .current_dir(repo_json["path"].as_str().unwrap())
            .args(cmd_args)
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
    println!("{}\n{}", title_string.to_uppercase(), (|| {
        "#".repeat(title_string.len())
    })());

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
