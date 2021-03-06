/*
题目：取一个整数a从右端开始的4〜7位。

程序分析：可以这样考虑：
(1)先使a右移4位。
(2)设置一个低4位全为1,其余全为0的数。可用~(~0<<4)
(3)将上面二者进行&运算。
 */
fn main() {
    println!("Hello, world!");

    let x = 0b110011110010011;

    println!("{:b}", !(!0 << 4) & (x >> 4));
}
