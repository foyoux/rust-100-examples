/*
题目：按逗号分隔列表。
 */
fn main() {
    println!("Hello, world!");

    let x = [1, 2, 3, 4, 5];

    let mut y = vec![];

    for i in x {
        y.push(i.to_string());
    }

    println!("{:?}", y.join(","));
}
