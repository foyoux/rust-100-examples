/*
题目：模仿静态变量(static)另一案例。

Rust 中静态变量的使用

 */

static mut STATIC_VAR: i32 = 0;

fn main() {
    println!("Hello, world!");

    unsafe {
        // 不安全代码，因为 STATIC_VAR 线程不安全
        println!("{}", STATIC_VAR);
        STATIC_VAR += 1;
        println!("{}", STATIC_VAR);
        STATIC_VAR += 1;
        println!("{}", STATIC_VAR);
        STATIC_VAR += 1;
        println!("{}", STATIC_VAR);
        STATIC_VAR += 1;
        println!("{}", STATIC_VAR);
    }
}
