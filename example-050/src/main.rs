/*
题目：输出一个随机数。
 */

use rand::Rng;

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();

    println!("{}", rng.gen::<i32>());
    println!("{}", rng.gen::<bool>());
    println!("{}", rng.gen::<i64>());
    println!("{}", rng.gen::<i8>());
    println!("{}", rng.gen::<f32>());
    println!("{}", rng.gen::<char>());
}
