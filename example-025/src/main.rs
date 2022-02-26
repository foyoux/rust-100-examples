/*
题目：求1+2!+3!+...+20!的和。
 */

fn factorial(n: u64) -> u64 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    return result;
}

fn main() {
    println!("Hello, world!");

    let mut sum = 0;
    for i in 1..=20 {
        let result = factorial(i);
        sum += result;
        println!("{}", result);
    }
    println!("sum = {}", sum);
}