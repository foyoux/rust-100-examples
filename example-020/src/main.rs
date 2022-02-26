/*
题目：一球从100米高度自由落下，每次落地后反跳回原高度的一半；再落下，求它在第10次落地时，共经过多少米？第10次反弹多高？
 */
fn main() {
    println!("Hello, world!");
    let mut h = 100.0;
    let mut sum = 0.0;

    for i in 0..10 {
        println!("{} {}", i, h);
        if i == 0 {
            sum += h;
        } else {
            sum += 2.0 * h;
        }
        h /= 2.0;
    }

    println!("{} {}", sum, h);
}
