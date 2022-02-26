/*
题目：打印出如下图案（菱形）:

   *
  ***
 *****
*******
 *****
  ***
   *
 */

fn main() {
    println!("Hello, world!");

    let n = 10;
    let row = 2 * n - 1;

    for i in 1..=row {
        let col = 2 * n - ((n - i) as i32).abs();

        for j in 1..col {
            if j <= ((n - i) as i32).abs() {
                print!(" ");
            } else {
                print!("*");
            }
        }

        println!();
    }
}
