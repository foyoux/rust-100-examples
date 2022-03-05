/*
题目：画图，学用line画直线。

[turtle](https://github.com/sunjay/turtle)
 */
use turtle::Turtle;

mod turtle;

fn main() {
    println!("Hello World");
    let mut turtle = Turtle::new();

    turtle.right(90.0);
    for _ in 0..100 {
        turtle.forward(2.0);
    }
}