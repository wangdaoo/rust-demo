// enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }

// fn main() {
//     // 使用 use 关键字将枚举引入作用域
//     // use TrafficLight::{Red, Yellow};
//     // let red = Red;
//     // let yellow = Yellow;
//     // let green = TrafficLight::Green;

//     // // 枚举的 match 表达式
//     // match red {
//     //     Red => println!("Stop!"),
//     //     Yellow => println!("Slow down!"),
//     //     TrafficLight::Green => println!("Go!"),
//     // }

//     // 枚举的 match 表达式
//     let light = TrafficLight::Green;
//     match light {
//         TrafficLight::Red => println!("Stop!"),
//         TrafficLight::Yellow => println!("Slow down!"),
//         TrafficLight::Green => println!("Go!Go!Go!"),
//     }
// }

// ------

// 带数据的枚举
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     // 枚举的方法
//     fn call(&self) {
//         // 处理不同类型的枚举
//         match self {
//             Message::Quit => println!("Quit"),
//             Message::Move { x, y } => println!("Move: {}, {}", x, y),
//             Message::Write(s) => println!("Write: {}", s),
//             Message::ChangeColor(r, g, b) => println!("ChangeColor: {}, {}, {}", r, g, b),
//         }
//     }
// }

// fn main() {
//     let msg = Message::Write(String::from("hello"));
//     msg.call();

//     let msg = Message::Move { x: 10, y: 20 };
//     msg.call();
// }

// ------ 递归枚举 ------
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

fn evaluate(e: &Expr) -> i32 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => evaluate(a) + evaluate(b),
        Expr::Sub(a, b) => evaluate(a) - evaluate(b),
    }
}

fn main() {
    // 创建表达式 (5 + 10) - 3
    let e = Expr::Sub(
        Box::new(Expr::Add(Box::new(Expr::Num(5)), Box::new(Expr::Num(10)))),
        Box::new(Expr::Num(3)),
    );

    // 计算表达式的值
    let result = evaluate(&e);
    println!("result: {}", result);
}
