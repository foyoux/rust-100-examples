/*
题目：暂停一秒输出。
 */
use std::thread;
use std::time::Duration;

fn main() {
    thread::sleep(Duration::from_millis(1000));
    println!("Hello, world!");
}
