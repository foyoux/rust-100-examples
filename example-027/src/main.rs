/*
题目：利用递归函数调用方式，将所输入的5个字符，以相反顺序打印出来。
 */

// 这个题目用 Rust 实现 很奇葩呀

use std::io::stdin;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("read_line error");
    foo(input.trim());
}

fn foo(s: &str) {
    if s.len() == 0 {
        return;
    }
    let chars = s.chars();
    let chars_len = chars.clone().count();
    let mut s = String::new();
    for (i, item) in chars.into_iter().enumerate() {
        if i == chars_len - 1 {
            print!("{}", item);
            break;
        }
        s.push(item);
    }
    foo(s.as_str());
}




