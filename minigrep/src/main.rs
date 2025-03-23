use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing args: {err}");
        process::exit(1);
    });

    // dbg!(&config);

    if let Err(e) = minigrep::run(config) {
        println!("app error: {e}");
        process::exit(1);
    }
}
