/*
题目：按相反的顺序输出列表的值。
 */
fn main() {
    println!("Hello, world!");

    let x = [1, 2, 3, 4, 5, 6];
    for i in (0..x.len()).rev() {
        println!("{}", x[i]);
    }
}
