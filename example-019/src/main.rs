/*
题目：一个数如果恰好等于它的因子之和，这个数就称为"完数"。例如6=1＋2＋3.编程找出1000以内的所有完数。
 */

fn main() {
    println!("Hello, world!");

    for i in 2..=1000 {
        let mut sum = 1;
        for j in 2..i {
            if i % j == 0 {
                sum += j;
            }
        }
        if sum == i {
            println!("{}", i);
        }
    }
}
