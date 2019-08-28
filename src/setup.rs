use json::JsonValue;
use std::{env, fs};
use std::fs::File;
use std::io::{Read, stdout, Write, stdin};

#[derive(Clone)]
pub struct SettingsFile {
    settings_file_path: String,
}

impl SettingsFile {
    pub fn new() -> SettingsFile {
        let settings_file_path = format!("{}/.pm.json", env::var("HOME").unwrap());

        let settings_file_res = File::open(settings_file_path.clone());

        match settings_file_res {
            Ok(t) => t,
            Err(_) => create_settings_file(settings_file_path.clone())
        };

        return SettingsFile {
            settings_file_path: settings_file_path.clone(),
        };
    }

    pub fn add_repo(self, new_repo: JsonValue) {
        let settings_json = get_settings_json(
            &mut File::open(
                self.settings_file_path.clone()
            ).unwrap()
        );

        let mut repos_mut = settings_json.clone();

        if !repos_mut["repos"].is_array() {
            repos_mut["repos"] = array![];
        }

        let mut new_repos = array![];

        for repo in repos_mut["repos"].members() {
            if repo["name"] != new_repo["name"] {
                new_repos.push(repo.clone()).unwrap();
            } else {
                new_repos.push(new_repo.clone());
            }
        }

        repos_mut["repos"] = new_repos;

        let settings_string = json::stringify(repos_mut);

        fs::write(
            self.settings_file_path.clone(),
            settings_string.as_bytes(),
        ).unwrap();
    }

    pub fn delete_repo(self, repo_name: &str) {
        let settings_json = get_settings_json(
            &mut File::open(
                self.settings_file_path.clone()
            ).unwrap()
        );
        let mut settings_mut = settings_json.clone();

        let members = settings_mut["repos"].members();
        let mut new_repos = array!();

        for member in members {
            if member["name"].as_str().unwrap() != repo_name {
                new_repos.push(member.clone()).unwrap();
            }
        }

        settings_mut["repos"] = new_repos;

        let settings_string = json::stringify(settings_mut);

        fs::write(
            self.settings_file_path.clone(),
            settings_string.as_bytes(),
        ).unwrap();
    }

    pub fn list_repos(self) -> JsonValue {
        let repos =
            get_settings_json(
                &mut File::open(self.settings_file_path)
                    .unwrap()
            )["repos"].clone();

        return repos;
    }

    pub fn get_settings_json(self) -> JsonValue {
        let mut file = File::open(self.settings_file_path).unwrap();
        get_settings_json(&mut file)
    }

    pub fn get_repo_by_name(self, repo_name: &str) -> JsonValue {
        let settings_json = get_settings_json(
            &mut File::open(
                self.settings_file_path.clone()
            ).unwrap()
        );

        let repo = settings_json["repos"].members().find(
            |r| {
                r["name"].as_str().unwrap() == repo_name
            }).unwrap();
        return (*repo).clone();
    }
}

fn create_settings_file(settings_file_path: String) -> File {
    let mut home_dir = String::new();
    print!("Input the path of where you clone your repos (default is $HOME/Desktop): ");
    stdout().flush().unwrap();
    stdin().read_line(&mut home_dir).unwrap();

    if home_dir == "\n" {
        home_dir = format!("{}/Desktop", env::var("HOME").unwrap());
    }
    home_dir = home_dir.replace("\n", "");
    let settings_json = object! {
            "defaultDir" => home_dir
        };
    let json_string = json::stringify(settings_json);

    fs::write(settings_file_path.clone(), json_string).unwrap();

    return File::open(settings_file_path.clone()).unwrap();
}

fn get_settings_json(settings_file: &mut File) -> JsonValue {
    let mut settings_string = String::new();
    settings_file.read_to_string(&mut settings_string).unwrap();

    let settings_json =
        json::parse(settings_string.as_str()).unwrap();
    return settings_json;
}
