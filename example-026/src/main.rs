/*
题目：利用递归方法求5!。
 */

fn foo(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }
    return n * foo(n - 1);
}

fn main() {
    println!("Hello, world!");

    println!("{}", foo(5));
}
