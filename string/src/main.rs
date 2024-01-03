fn main() {
    // println!("Hello, world!");
    let first_name = String::from("John");
    let last_name = String::from("Doe");

    // 使用 + 运算符连接字符串
    // let full_name = first_name + "·" + &last_name;

    // println!("Full Name: {}", full_name); // Full Name: John·Doe

    // 使用 format! 宏连接字符串
    let full_name = format!("{}·{}", first_name, last_name);

    println!("Full Name: {}", full_name); // Full Name: John·Doe

    let number_str = "10";

    // 将字符串转换为 i32 类型
    let number: i32 = number_str.parse().expect("Not a number!");

    println!("Number: {}", number); // 打印 Number: 10

    // ----------------------------
    let name = "John";
    greet(name.to_string());
}

fn greet(name: String) {
    println!("Hello, {}!", name);
}

// fn tt() {
//     let s = String::from("hello world");
//     let s1: &str = "hello world";

//     println!("s: {}", s);
//     println!("s1: {}", s1);
// }
