/*
题目：求输入数字的平方，如果平方运算后小于 50 则退出。

程序分析：无
 */

use std::io::stdin;

fn read_num() -> i32 {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("read_line error");

    return input.trim().parse().unwrap();
}

fn main() {
    println!("Hello, world!");

    loop {
        let num = read_num();
        let num_num = num * num;
        println!("{} ^ 2 = {}", num, num_num);

        if num_num < 50 {
            break;
        }
    }
}
