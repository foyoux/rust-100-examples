/*
题目：输入某年某月某日，判断这一天是这一年的第几天？

程序分析：以3月5日为例，应该先把前两个月的加起来，然后再加上5天即本年的第几天，特殊情况，闰年且输入月份大于2时需考虑多加一天：
 */
use std::io::stdin;

fn main() {
    const DAYS_OF_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    println!("请输入年月日（以/分隔）：");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("读取输入失败");

    let date: Vec<&str> = input.trim().split("/").collect();
    assert_eq!(date.len(), 3, "输入不符合规范");

    let year: u16 = date[0].parse().expect("输入不符合规范");
    let month: u8 = date[1].parse().expect("输入不符合规范");
    let day: u8 = date[2].parse().expect("输入不符合规范");


    let mut days = if is_leap(year) { 1 } else { 0 } + day;

    for i in 0..usize::from(month - 1) {
        days += DAYS_OF_MONTH[i];
    }

    println!("{}/{}/{} 是今年的第 {} 天 ", year, month, day, days);
}

fn is_leap(year: u16) -> bool {
    if year % 4 == 0 && year % 100 != 0 {
        return true;
    }
    return false;
}
