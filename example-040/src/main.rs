/*
题目：将一个数组逆序输出。

程序分析：用第一个与最后一个交换。
 */
fn main() {
    println!("Hello, world!");

    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for i in (0..array.len()).rev() {
        println!("{}", array[i]);
    }
}
