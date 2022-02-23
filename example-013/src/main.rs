/*
题目：打印出所有的"水仙花数"，所谓"水仙花数"是指一个三位数，其各位数字立方和等于该数本身。例如：153是一个"水仙花数"，因为153=1的三次方＋5的三次方＋3的三次方。

程序分析：利用for循环控制100-999个数，每个数分解出个位，十位，百位。
 */

fn is_daffodils_num(n: u64) -> bool {
    if n < 100 || n > 999 {
        return false;
    }

    let a = n / 100;
    let b = n / 10 % 10;
    let c = n % 10;

    let m = a.pow(3) + b.pow(3) + c.pow(3);

    if m == n {
        return true;
    }
    return false;
}

fn main() {
    println!("Hello, world!");
    for i in 100..=999 {
        if is_daffodils_num(i) {
            println!("{}", i);
        }
    }
}
