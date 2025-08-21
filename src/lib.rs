use std::process;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn new(query: String, file_path: String, ignore_case: bool) -> Self {
        Config { query, file_path, ignore_case }
    }

    pub fn build() -> Result<Config, &'static str> {
        let args: Vec<String> = std::env::args().collect();
        if args.len() <3 {
        println!("Usage: minigrep <query> <file_path>");   
        process::exit(1)     }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // Example: use an env var for ignore_case
        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config::new(query, file_path, ignore_case))
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    println!("With Text:\n{contents}");

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    println!("{:?}", results);
    Ok(())
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| line.trim())
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .map(|line| line.trim())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive_test() {
        let query = "Hello";
        let contents = "The world is gone Hello\nHello can you hear me,Hello?";

        let output = search_case_sensitive(query, contents);

        assert_eq!(output, vec!["The world is gone Hello", "Hello can you hear me,Hello?"]);
    }

    #[test]
    fn search_case_insensitive_test() {
        let query = "hello";
        let contents = "The world is gone Hello\nHello can you hear me,Hello?";

        let output = search_case_insensitive(query, contents);

        assert_eq!(output, vec!["The world is gone Hello", "Hello can you hear me,Hello?"]);
    }
}
