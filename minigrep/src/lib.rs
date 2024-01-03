// src/lib.rs
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// #[derive(Debug)]：
//  这是一个属性（attribute），它自动为 ConfigError 枚举实现了 Debug trait
//  使得枚举可以被格式化输出，便于调试。
#[derive(Debug)]
pub enum ConfigError {
    NoSearchString,
    NoFilename,
}

// 实现 Display trait，使得 ConfigError 可以被格式化输出
// Display trait 要求实现一个名为 fmt 的方法，它使用 write! 宏（而不是 println! 宏）向标准输出进行格式化
impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::NoSearchString => write!(f, "未指定搜索字符串"),
            ConfigError::NoFilename => write!(f, "未指定文件名"),
        }
    }
}

impl Error for ConfigError {}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, ConfigError> {
        // 第一个参数是程序名，跳过
        args.next();

        // 搜索的字符串
        let query = match args.next() {
            Some(arg) => arg,
            // None => return Err("没有搜索字符串"),
            None => return Err(ConfigError::NoSearchString),
        };

        // 文件名
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err(ConfigError::NoFilename),
        };

        // 是否区分大小写
        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    // 使用函数式编程改写
    contents
        .lines()
        .filter(|line| {
            if case_sensitive {
                line.contains(query)
            } else {
                line.to_lowercase().contains(&query.to_lowercase())
            }
        })
        .collect()

    /*
    lines
        将目标字符串按行分割，返回一个迭代器
    to_lowercase
        将字符串转换为小写
    filter
        过滤器，传入一个闭包，闭包的参数是迭代器的每个元素，返回一个布尔值
        如果闭包返回 true，元素会被保留，否则会被丢弃
    contains
        判断字符串是否包含某个子串
    collect
        将迭代器转换为一个集合
     */
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件
    let contents = fs::read_to_string(config.filename)?;
    // println!("With text:\n{}", contents);

    for line in search(&config.query, &contents, config.case_sensitive) {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        // let contents = fs::read_to_string("poem.txt").unwrap();

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }
}
