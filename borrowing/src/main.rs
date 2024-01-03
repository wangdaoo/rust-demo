fn main() {
    // let x = 5;
    // let y = &x;

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2: String = String::from("hello");
    let len: usize = calculate_length2(&mut s2);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(", world!"); // error!
    s.len()
}

fn calculate_length2(s: &mut String) -> usize {
    s.push_str(", world!");
    s.len()
}
