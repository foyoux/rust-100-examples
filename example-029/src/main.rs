/*
题目：给一个不多于5位的正整数，要求：一、求它是几位数，二、逆序打印出各位数字。

程序分析：学会分解出每一位数。
 */
fn main() {
    println!("Hello, world!");

    let n: i32 = 123456;
    let s = n.to_string();

    for i in s.chars().rev() {
        println!("{}", i);
    }
}

