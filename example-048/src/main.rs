/*
题目：数字比较。

程序分析：无
 */
use std::io::stdin;

fn main() {
    println!("Hello, world!");

    let x = read_num();
    let y = read_num();

    if x == y {
        println!("{} == {}", x, y);
    } else {
        match x > y {
            true => println!("{} > {}", x, y),
            false => println!("{} < {}", x, y),
        };
    }
}


fn read_num() -> i32 {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("read_line error");

    return input.trim().parse().unwrap();
}