// src/main.rs
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // 将命令行参数收集到一个字符串向量中
    let args: Vec<String> = env::args().collect();

    // 打印出命令行参数
    println!("{:?}", args);

    /*
    当我们调用 unwrap 或 expect 时，如果 Result 值是 Err，这两个方法都会 panic！
    这就是为什么我们在 main 函数中不直接调用 unwrap，而是在 main 函数中调用一个新的函数，
    这样当出现错误时，我们就可以选择在 main 函数中处理错误，而不是在调用函数的代码中处理错误。

    当 Result 包含错误时，不再调用 panic! 让程序奔溃，而是返回一个 Err 值，这个值会传播到调用的代码中。
    通过 process::exit(1) 退出程序，这个数字是操作系统的错误代码，非零的退出码表明程序因为某种方式失败了。

    unwrap_or_else 方法获取一个闭包作为参数，而不是一个直接返回 Err 分支的函数。
        如果 Result 是 Ok，闭包中的代码不会执行。
        如果 Result 是 Err，闭包中的代码会被执行，并且 Err 的内容会传递给闭包中的参数 err。

    unwrap_or_else 和 expect 的选择：
        如果遇到错误，你希望程序停止运行，调用 unwrap 或 expect。
        如果遇到错误，你希望程序执行一些其他操作，调用 unwrap_or_else。
     */
    // let config = Config::build(&args).unwrap_or_else(|err| {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // 如果出错，打印错误信息
        eprintln!("Problem parsing arguments: {}", err);
        // 退出程序
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

// Usage: cargo run search_string filename
