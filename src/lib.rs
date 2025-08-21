use std::error::Error;
use std::fs;

pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    let config: Config = (query, file_path).into();
    config
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    fn new(query: String, file_path: String) -> Self {
        Config { query, file_path }
    }

    pub fn build() -> Result<Config, &'static str> {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 3 {
            return Err("Did not pass neccesary arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config::from((query, file_path)))
    }
}

impl From<(String, String)> for Config {
    fn from(value: (String, String)) -> Config {
        let (query, file_path) = value;
        Config::new(query, file_path)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path)?;
    println!("With Text:\n{contents}");
    Ok(())
}
