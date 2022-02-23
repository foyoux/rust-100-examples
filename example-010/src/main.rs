/*
题目：暂停一秒输出，并格式化当前时间。
 */

use std::ops::Add;
use std::thread;
use std::time::Duration;
use chrono;

fn main() {
    println!("Hello, world!");
    thread::sleep(Duration::from_millis(1000));
    let now = chrono::Local::now();
    println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
    println!("{}", now);
}
