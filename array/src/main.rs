fn main() {
    // 数组长度固定，存储在栈上
    let arr = [1, 2, 3, 4, 5];
    println!("arr[0] = {}", arr[0]); // arr[0] = 1

    // 遍历数组
    for i in 0..arr.len() {
        println!("arr[{}] = {}", i, arr[i]);
    }

    // 数组切片
    let arr_slice = &arr[1..3];
    println!("arr_slice[0] = {}", arr_slice[0]); // arr_slice[0] = 2
}
