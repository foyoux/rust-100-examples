/*
题目：请输入星期几的第一个字母来判断一下是星期几，如果第一个字母一样，则继续判断第二个字母。

程序分析：用情况语句比较好，如果第一个字母一样，则判断用情况语句或if语句判断第二个字母。。

Sunday      星期日
Monday      星期一
Tuesday     星期二
Wednesday   星期三
Thursday    星期四
Friday      星期五
Saturday    星期六

 */
use std::io::stdin;

fn main() {
    println!("Hello, world!");

    match read_line().as_str() {
        "S" => {
            match read_line().as_str() {
                "u" => println!("星期日"),
                "a" => println!("星期六"),
                _ => println!("UNKNOWN"),
            }
        }
        "M" => println!("星期一"),
        "T" => {
            match read_line().as_str() {
                "u" => println!("星期二"),
                "h" => println!("星期四"),
                _ => println!("UNKNOWN"),
            }
        }
        "W" => println!("星期三"),
        "F" => println!("星期五"),
        _ => println!("UNKNOWN"),
    }
}


fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("read_line error");
    return input.trim().to_string();
}