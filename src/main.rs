use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;

use glob::glob_with;
use glob::MatchOptions;
use regex::Regex;

use clap::{command, Command};

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

fn init_subcommand() {
    println!("Entered `init` subcommand");
    let mut env_vars: HashMap<String, String> = HashMap::new();

    let options = MatchOptions {
        case_sensitive: true,
        require_literal_separator: false,
        require_literal_leading_dot: true,
    };

    let env_file_re = Regex::new(r"\.env(\.[^.]*)?$").unwrap();
    let export_re = Regex::new(r"^([A-Za-z_][A-Za-z0-9_]*)=(.+)$").unwrap();
    for entry in glob_with(".env*", options).expect("Failed to read glob operation") {
        match entry {
            Ok(path) => {
                let path_str = path.to_str().unwrap();
                if env_file_re.is_match(path_str) {
                    if let Ok(file_data) = read_file_lines(path_str) {
                        for line in file_data.flatten() {
                            if export_re.is_match(&line) {
                                println!("{:?}", line);
                                let arr: Vec<String> =
                                    line.split('=').map(|s| s.to_string()).collect();
                                env_vars.insert(arr[0].clone(), arr[1].clone());
                            }
                        }
                    }
                }
            }
            Err(e) => println!("{:}", e),
        }
    }
    println!("{:?}", env_vars);
    hasmap_to_file(env_vars, "test.json")
}
fn read_file_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn hasmap_to_file(hashmap: HashMap<String, String>, filename: &str) {
    let json_string = serde_json::to_string(&hashmap).unwrap();

    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(json_string.as_bytes())
        .expect("Failed to write to file");
}
