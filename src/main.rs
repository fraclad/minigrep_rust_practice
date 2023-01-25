use std::env; 
use std::process;
use std::fs;
use colored::Colorize;

fn main() {
    let arg_input: Vec<String> = env::args().collect();
    let config = Config::new(&arg_input).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });
    let text = read_file(&config.filename);
    let result = run(&config, &text);
    if result.len() == 0 {
        println!("{}", "no line matches the given query".red())
    } else {
        println!("{:#?}", result)
    }
}

fn run<'a>(config: &Config, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    if config.is_case_sensitive {
        result = search_case_sensitive(&config.query, &content);
    } else {
        result = search_case_insensitive(&config.query, &content);
    }
    result
}

fn read_file(filename: &str) -> String {
    let file_content = fs::read_to_string(filename).expect("cannot read file");
    file_content
}

fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in content.lines(){
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines(){
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }
    result
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
