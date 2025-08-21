use minigrep::Config;
use minigrep::run;
use std::process;

fn main() {
    let config = Config::build().unwrap_or_else(|e| {
        println!("Errro {e}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) =run(config){
        println!("Application Error {e}");
        process::exit(1);
    }
}
