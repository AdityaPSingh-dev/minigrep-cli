use std::env;
extern crate minigrep;
use minigrep::Config;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|_err| {
        eprintln!("Problem parsing arguments: {}", _err);
        process::exit(1);
    });
    if let Err(_e) = minigrep::run(config) {
        //as success return only () i.e. empty that's why only
        // checking Error and not using unwrap_or_else
        eprintln!("Problem parsing arguments: {}", _e);
        process::exit(1);
    }
}
