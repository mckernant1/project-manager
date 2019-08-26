use json::JsonValue;
use std::{env, fs};
use std::fs::File;
use std::io::{Read, stdout, Write, stdin};

pub fn get_settings_json() -> JsonValue {
    let settings_file_path = format!("{}/.gg.json", env::var("HOME").unwrap());
    let mut settings_file =
        File::open(settings_file_path.clone())
            .unwrap_or(
                create_settings_file(settings_file_path.clone())
            );

    let mut settings_string = String::new();

    settings_file.read_to_string(&mut settings_string).unwrap();

    let settings_json = json::parse(settings_string.as_str()).unwrap();

    return settings_json;
}

fn create_settings_file(settings_file_path: String) -> File {

    let mut home_dir = String::new();
    print!("Input the path of where you clone your repos (default is $HOME/Desktop): ");
    stdout().flush();
    stdin().read_line(&mut home_dir).unwrap();

    if home_dir == "\n" {
        home_dir = format!("{}/Desktop", env::var("HOME").unwrap());
    }
    home_dir = home_dir.replace("\n", "");
    let settings_json = object!{
        "defaultDir" => home_dir
    };
    let json_string = json::stringify(settings_json);

    fs::write(settings_file_path.clone(), json_string).unwrap();

    return File::open(settings_file_path.clone()).unwrap();
}
