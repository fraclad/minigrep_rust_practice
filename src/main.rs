use std::env; 
use std::process;
use std::fs;
use std::error::Error;

fn main() {
    let arg_input: Vec<String> = env::args().collect();
    let config = Config::new(&arg_input).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });
    let text = read_file(&config.filename);
}

fn read_file(filename: &str) -> String {
    let file_content = fs::read_to_string(filename).expect("cannot read file");
    println!("text : \n{}", file_content);
    file_content
}

#[derive(Debug)]
struct Config {
    query: String,
    filename: String,
    is_case_sensitive: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 4 {
            return Err("not enough arguments!")
        }

        let mut is_case_sensitive_parse: bool = true;
        if args[1].clone().trim().to_lowercase() != String::from("true") {
            is_case_sensitive_parse = false
        }

        Ok(Config {
            query: args[2].clone(),
            filename: args[3].clone(),
            is_case_sensitive: is_case_sensitive_parse,
        })
    }
}
