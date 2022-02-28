/*
题目：文本颜色设置。
 */
use ansi_term::Colour;

fn main() {
    println!("\x1B[31mHello, world!\x1B[0m");

    println!("This is {} in color, {} in color and {} in color",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));
}
