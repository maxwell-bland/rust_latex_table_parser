// Rust Command Line Tool that Parses latex files
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tex_parser::ast::*;
use tex_parser::parse;

mod newcommand_hashmap;
mod tabular_handlers;

// function to read in list of latex files from command line
fn read_files(args: &Vec<String>) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for arg in args {
        files.push(arg.to_string());
    }
    files
}

// function to read in contents of a file
fn read_file(file: &String) -> String {
    let path = Path::new(file);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.to_string()),
        Ok(_) => s,
    }
}

// main function to parse command line args
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // remove the first argument
    let args = args
        .iter()
        .skip(1)
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let files = read_files(&args);
    // Get the sets of tokens from each file
    let mut tokens: Vec<Vec<Token>> = Vec::new();
    for file in files {
        tokens.push(parse(&read_file(&file)).unwrap().content);
    }
    // combine the newcommand hashmaps extracted from all the files
    let mut newcommand_hashmaps: Vec<HashMap<String, String>> = Vec::new();
    for toks in &tokens {
        newcommand_hashmaps.push(newcommand_hashmap::get_newcommand_hashmap(&toks));
    }

    // extract out all the tabular environments from the files
    for toks in &tokens {
        let tabular_environments = tabular_handlers::get_environments(&toks, "tabular");
        // get tokens for each tabular environment
        for environment in tabular_environments {
            let tokens = environment.body.clone();
            // for each tabular environment, get the contents of the tabular environment
            let rows = tabular_handlers::parse_tabular(&tokens);

            // print the data in each of the table rows, substituting newcommand values
            for row in &rows {
                for c in row {
                    let mut printed = false;
                    if newcommand_hashmaps.len() > 0 {
                        for newcommand_hashmap in &newcommand_hashmaps {
                            if newcommand_hashmap.contains_key(c) {
                                print!(" {} ", newcommand_hashmap.get(c).unwrap());
                                printed = true;
                            }
                        }
                    }
                    if !printed {
                        print!(" {} ", c);
                    }
                }
                println!("");
            }
        }
    }
}
