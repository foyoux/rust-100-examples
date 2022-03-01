/*
题目：学习使用按位与 & 。

程序分析：0&0=0; 0&1=0; 1&0=0; 1&1=1。
 */
fn main() {
    println!("Hello, world!");

    let x = 0;
    let y = 1;

    println!("{} & {} = {}", x, y, x & y);
    println!("{} & {} = {}", x, x, x & x);
    println!("{} & {} = {}", y, y, y & y);

    println!("============");
    println!("{} | {} = {}", x, y, x | y);
    println!("{} | {} = {}", x, x, x | x);
    println!("{} | {} = {}", y, y, y | y);

    println!("============");
    println!("{} ^ {} = {}", x, y, x ^ y);
    println!("{} ^ {} = {}", x, x, x ^ x);
    println!("{} ^ {} = {}", y, y, y ^ y);

}
