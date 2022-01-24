mod algorithm;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("<ALGORITHM_CODE> is required.");
        std::process::exit(1)
    }

    let algorithm_code = &args[1];
    run(algorithm_code);
}

fn run(code: &str) {
    match code {
        "a1" => algorithm::algorithm_a::a1::run(),
        _ => println!("algorithm '{}' is not found.", code),
    }
}
