/*
题目：使用lambda来创建匿名函数。

程序分析：无
 */
fn main() {
    println!("Hello, world!");

    println!("{} from lambda",
             (
                 || {
                     "Hell Rust"
                 }
             )()
    );
}
