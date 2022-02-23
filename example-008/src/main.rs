/*
题目：输出 9*9 乘法口诀表。

程序分析：分行与列考虑，共9行9列，i控制行，j控制列。
 */
fn main() {
    println!("Hello, world!");
    for i in 1..=9 {
        for j in 1..=i {
            print!("{} × {} = {}\t", j, i, i * j);
        }
        println!();
    }
}
