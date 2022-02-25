/*
题目：输入一行字符，分别统计出其中英文字母、空格、数字和其它字符的个数。
 */
use std::io::stdin;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut char_count = 0;
    let mut num_count = 0;
    let mut space_count = 0;

    for i in input.chars() {
        if i.is_ascii_digit() {
            num_count += 1;
        } else if i.is_ascii_whitespace() {
            space_count += 1;
        } else if i.is_ascii() {
            char_count += 1;
        }
    }

    println!("ascii {}, num {}, space {}", char_count, num_count, space_count - 1);
}
