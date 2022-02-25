/*
题目：利用条件运算符的嵌套来完成此题：学习成绩>=90分的同学用A表示，60-89分之间的用B表示，60分以下的用C表示。

程序分析：程序分析：(a>b) ? a:b 这是条件运算符的基本例子。
 */
use std::io::stdin;

fn main() {
    println!("Hello, world!");

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let score: u32 = line.trim().parse().unwrap();
    let rate = match score {
        x if x >= 90 => 'A',
        x if x >= 60 => 'B',
        _ => 'C'
    };

    println!("你的评级 {}", rate);
}


