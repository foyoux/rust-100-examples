/*
题目：练习函数调用。
 */

fn hello(name: &str) {
    println!("hello {} ~", name);
}

fn main() {
    println!("Hello, world!");

    hello("rust");
}
