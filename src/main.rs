use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;

use clap::{command, Command};
use glob::glob_with;
use glob::MatchOptions;
use regex::Regex;
use serde::Serialize;

fn main() {
    let _matches = command!() // requires `cargo` feature
        .about("Environment variable management with superpowers.")
        // .arg(arg!([name] "Optional name to operate on"))
        // .arg(
        //     Arg::new("init")
        //         .long("init")
        //         .short('i')
        //         .help("Initialises the current directory"),
        // )
        .subcommand(Command::new("init").about("Initialise superenv in the current directory."))
        .get_matches();

    if let Some(_init_subcommand) = _matches.subcommand_matches("init") {
        init_subcommand();
    }
}

#[derive(Serialize)]
struct EnvVarIndex {
    key: String,
    environment: String,
}

fn init_subcommand() {
    println!("Entered `init` subcommand");
    let mut env_vars: HashMap<EnvVarIndex, String> = HashMap::new();

    let options = MatchOptions {
        case_sensitive: true,
        require_literal_separator: false,
        require_literal_leading_dot: true,
    };

    let env_file_re = Regex::new(r"\.env(\.[^.]*)?$").unwrap();
    let export_re = Regex::new(r"^([A-Za-z_][A-Za-z0-9_]*)=(.+)$").unwrap();

    let mut seen_base: bool = false;

    for entry in glob_with(".env*", options).expect("Failed to read glob operation") {
        match entry {
            Ok(path) => {
                let path_str = path.to_str().unwrap();
                if env_file_re.is_match(path_str) {
                    // we have a  valid env file
                    let parts: Vec<&str> = path_str.split(".").collect();

                    let environment = match parts.get(1) {
                        // if an index of 1 returns none then we have the base
                        Some(&s) => s,
                        None => "base",
                    };

                    if let Ok(file_data) = read_file_lines(path_str) {
                        for line in file_data.flatten() {
                            if export_re.is_match(&line) {
                                // we have a valid environment variable export
                                //
                                let arr: Vec<String> =
                                    line.split('=').map(|s| s.to_string()).collect();

                                let key_to_insert = EnvVarIndex {
                                    key: arr[0].clone(),
                                    environment: environment.to_string(),
                                };
                            }
                        }
                    }
                }
            }
            Err(e) => println!("{:}", e),
        }
    }
    hasmap_to_file(env_vars, "test.json")
}
fn read_file_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn hasmap_to_file(hashmap: HashMap<EnvVarIndex, String>, filename: &str) {
    let json_string = serde_json::to_string(&hashmap).unwrap();

    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(json_string.as_bytes())
        .expect("Failed to write to file");
}
