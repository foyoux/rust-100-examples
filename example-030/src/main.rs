/*
题目：一个5位数，判断它是不是回文数。即12321是回文数，个位与万位相同，十位与千位相同。

程序分析：无。
 */
fn main() {
    println!("Hello, world!");
    let n = 22321;
    println!("{} 是回文数 {}", n, foo(n));
    let n = 12321;
    println!("{} 是回文数 {}", n, foo(n));
}

fn foo(n: u32) -> bool {
    let tmp = n.to_string();
    let x = tmp.as_bytes();
    return if x[0] == x[4] && x[1] == x[3] {
        true
    } else {
        false
    };
}
