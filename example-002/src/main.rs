/*
题目：企业发放的奖金根据利润提成。利润(I)低于或等于10万元时，奖金可提10%；利润高于10万元，低于20万元时，低于10万元的部分按10%提成，高于10万元的部分，可提成7.5%；20万到40万之间时，高于20万元的部分，可提成5%；40万到60万之间时高于40万元的部分，可提成3%；60万到100万之间时，高于60万元的部分，可提成1.5%，高于100万元时，超过100万元的部分按1%提成，从键盘输入当月利润I，求应发放奖金总数？

程序分析：请利用数轴来分界，定位。
*/

use std::io;

fn get_profit() -> u32 {
    let mut profit = String::new();
    io::stdin().read_line(&mut profit).expect("读取标准输入失败");
    let profit: u32 = profit.trim().parse().expect("转换输入为数值型失败");
    return profit;
}

fn main() {
    let arr1 = [1000000, 600000, 400000, 200000, 100000, 0];
    let arr2 = [0.01, 0.015, 0.03, 0.05, 0.075, 0.1];

    println!("请输入利润: ");
    let mut profit = get_profit();

    let mut award = 0.0;
    for i in 0..6 {
        let x1 = arr1[i];
        let x2 = arr2[i];
        if profit > x1 {
            award += x2 * (profit - x1) as f64;
            profit = x1;
        }
    }
    println!("应发奖金: {}", award);
}
