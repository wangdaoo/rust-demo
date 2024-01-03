// src/lib.rs
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::io;

#[derive(Debug)]
pub enum ConfigError {
    NoSearchString,
    NoFilename,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::NoSearchString => write!(f, "未指定搜索字符串"),
            ConfigError::NoFilename => write!(f, "未指定文件"),
        }
    }
}

impl Error for ConfigError {}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filenames: Vec<String>,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(
        query: String,
        filenames: Vec<String>,
        case_sensitive: bool,
    ) -> Result<Config, ConfigError> {
        if query.is_empty() {
            return Err(ConfigError::NoSearchString);
        }

        if filenames.is_empty() {
            return Err(ConfigError::NoFilename);
        }

        Ok(Config {
            query,
            filenames,
            case_sensitive,
        })
    }
}

pub fn init_config() -> Result<Config, Box<dyn Error>> {
    // 查询字符串
    let query: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("请输入查询字符串")
        .interact_text()?;
    if query.is_empty() {
        return Err(Box::new(ConfigError::NoSearchString));
    }

    // 文件名
    let files = get_files_form_current_dir()?;
    if files.is_empty() {
        return Err(Box::new(ConfigError::NoFilename));
    }

    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("请选择文件")
        .items(&files[..])
        .interact()?;

    let filenames: Vec<String> = selection
        .iter()
        .map(|index| files[*index].clone())
        .collect();

    let selections: &[&str; 2] = &["大小写敏感", "大小写不敏感"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("请选择大小写敏感")
        .default(0)
        .items(&selections[..])
        .interact()?;

    let case_sensitive: bool = match selection {
        0 => true,
        1 => false,
        _ => true,
    };

    Ok(Config::new(query, filenames, case_sensitive)?)
}

pub fn search(config: Config) {
    for filename in config.filenames {
        let contents = fs::read_to_string(&filename).unwrap_or_else(|err| {
            eprintln!("读取文件 {} 失败：{}", filename, err);
            std::process::exit(1);
        });

        for line in contents.lines() {
            if (config.case_sensitive && line.contains(&config.query))
                || (!config.case_sensitive
                    && line.to_lowercase().contains(&config.query.to_lowercase()))
            {
                println!("{}", line.bright_green().on_bright_yellow());
            }
        }
    }
}

pub fn get_files_form_current_dir() -> io::Result<Vec<String>> {
    let mut files: Vec<String> = Vec::new();
    let paths = fs::read_dir("./")?;

    for path in paths {
        let path = path?.path();
        let path = path.to_str().unwrap();
        // 判断是否是文件
        let metadata = fs::metadata(path)?;
        if !metadata.is_file() {
            continue;
        }
        // 如果是隐藏文件，跳过
        if path.starts_with("./.") {
            continue;
        }
        // 如果是二进制文件，跳过
        if path.ends_with(".exe") {
            continue;
        }

        // push 进去删除 ./ 的路径, 例如：./Cargo.toml -> Cargo.toml
        files.push(path[2..].to_string());
    }

    Ok(files)
}
