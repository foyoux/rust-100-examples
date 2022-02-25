/*
题目：输出指定格式的日期。
 */

use chrono::{Datelike, Local};

fn main() {
    println!("Hello, world!");

    let now = Local::now();

    println!("{}/{}/{}", now.year(), now.month(), now.day());
}
