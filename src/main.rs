use std::{env, process};

use minigrep::Config;

fn main() {
    // env::args() 호출로 반환된 Iterator의 소유권을 Config::build로 전달
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // 에러가 발생할 경우 클로저 내부의 코드가 실행됨
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
