/*
题目：求一个3*3矩阵主对角线元素之和。

程序分析：利用双重for循环控制输入二维数组，再将a[i][i]累加后输出
 */

use std::io::stdin;

fn main() {
    println!("Hello, world!");
    let mut x: Vec<[i32; 3]> = vec![];

    for _ in 0..3 {
        x.push(read_nums());
    }

    let mut sum = 0;

    for i in 0..3 {
        sum += x[i][i];
    }

    println!("{}", sum);
}

fn read_nums() -> [i32; 3] {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("read_line error");

    let num_strings: Vec<&str> = input.trim().split(' ').collect();
    let mut nums = [0; 3];

    for i in 0..nums.len() {
        nums[i] = num_strings[i].parse().unwrap();
    }

    return nums;
}
