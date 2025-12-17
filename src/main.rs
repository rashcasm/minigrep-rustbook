use std::env;
use std::fs;
use std::error::Error;
use minigrep::search;

fn main() {
    // part 1 - accepting command line arguments
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Error message: {err}");
        std::process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // part 2 - reading a file
    // let contents = fs::read_to_string(file_path)
    //     .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
    
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        std::process::exit(1);
    }

}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // println!("With text:\n{contents}");
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}







// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             panic!("not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Config { query, file_path }
//     }
// }

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let file_path = &args[2];

//     (query, file_path)
// }