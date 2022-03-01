/*
题目：两个变量值互换。

程序分析：无
 */
fn main() {
    println!("Hello, world!");

    let mut x = 23;
    let mut y = 543;

    println!("x is {}, y is {}", x, y);

    x = x ^ y;
    y = x ^ y;
    x = x ^ y;

    println!("x is {}, y is {}", x, y);
}
