#[macro_use]
extern crate clap;
#[macro_use]
extern crate json;

mod setup;

use clap::{App, AppSettings};
use std::{env, io, fs};
use std::fs::File;
use std::io::{Read, stdin, stdout, Write};
use json::JsonValue;
use setup::get_settings_json;



fn main() {
    let settings_json = get_settings_json();


    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();
}


