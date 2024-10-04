use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // String 슬라이스(연속된 String 요소를 참조)를 인자로 받음
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// Box<dyn Error> - Error 트레이트를 구현한 타입을 반환
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // search 함수에서 반환된 문자열 슬라이스 벡터를 반복하여 각 라인을 출력
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

// 라이프타임을 명시하여 반환되는 벡터의 문자열 슬라이스가 contents 문자열 슬라이스의 참조임을 나타냄
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
