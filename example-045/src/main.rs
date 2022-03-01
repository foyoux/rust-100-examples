/*
题目：统计 1 到 100 之和。

程序分析：无

 */
fn main() {
    println!("Hello, world!");

    let mut sum = 0;

    for i in 1..=100 {
        sum += i;
    }

    println!("1 + 2 + ... + 100 = {}", sum);
}
