// src/main.rs
use minigrep_with_clap as minigrep;

fn main() {
    match minigrep::init_config() {
        Ok(config) => {
            minigrep::search(config);
        }
        Err(err) => {
            eprintln!("初始化 Config 失败：{}", err);
            std::process::exit(1);
        }
    }
}
