/*
题目：判断101-200之间有多少个素数，并输出所有素数。

程序分析：判断素数的方法：用一个数分别去除2到sqrt(这个数)，如果能被整除，则表明此数不是素数，反之是素数。
 */

fn sqrt_ceil(n: u64) -> u64 {
    return (n as f64).sqrt().ceil() as u64;
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..sqrt_ceil(n) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

fn main() {
    println!("Hello, world!");

    for i in 101..=200 {
        if is_prime(i) {
            println!("{}", i);
        }
    }
}
