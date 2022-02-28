/*
题目：求100之内的素数和。
 */

fn main() {
    println!("Hello, world!");

    let mut sum = 0u32;

    for i in 0..=100 {
        sum += if is_prime(i) {
            i
        } else { 0 }
    }

    println!("{}", sum);
}

fn is_prime(n: u32) -> bool {
    if n < 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
