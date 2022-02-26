/*
题目：求s=a+aa+aaa+aaaa+aa...a的值，其中a是一个数字。例如2+22+222+2222+22222(此时共有5个数相加)，几个数相加由键盘控制。
 */

use std::io::stdin;

fn input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut line = String::new();
    stdin().read_line(&mut line);

    line.trim().to_string()
}

fn int_input(prompt: &str) -> u64 {
    input(prompt).parse().unwrap()
}

fn main() {
    println!("Hello, world!");

    let n = int_input("请输入 n:");
    let a = int_input("请输入 a:");

    let mut sum = 0;
    let mut val = 0;
    for i in 0..n {
        val = a * 10u64.pow(i as u32) + val;
        sum += val;
        if i + 1 == n {
            println!("{} = {}", val, sum);
        } else {
            print!("{} + ", val);
        }
    }
}
