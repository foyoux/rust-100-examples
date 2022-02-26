/*
题目：有一分数序列：2/1，3/2，5/3，8/5，13/8，21/13...求出这个数列的前20项之和。

程序分析：请抓住分子与分母的变化规律。
 */
fn main() {
    println!("Hello, world!");
    let mut son = 1.0;
    let mut mother = 2.0;
    let mut sum = 0.0;

    for _ in 0..20 {
        sum += mother / son;

        let tmp = mother + son;
        son = mother;
        mother = tmp;
    }

    println!("{}", sum);
}
