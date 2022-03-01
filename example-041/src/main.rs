/*
题目：将一个数组逆序输出。

程序分析：用第一个与最后一个交换。

这题真没质量呀~ 这原本是C语言的100题目，转而为Python，现在我用作Rust. 后期慢慢改成Rust相关的。
 */
fn main() {
    println!("Hello, world!");

    let mut vec = vec![1, 2, 3, 4, 5, 6, 87];

    vec.reverse();

    println!("{:?}", vec);
}
