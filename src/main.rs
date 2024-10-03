use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        // 에러가 발생할 경우 클로저 내부의 코드가 실행됨
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // String 슬라이스(연속된 String 요소를 참조)를 인자로 받음
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
