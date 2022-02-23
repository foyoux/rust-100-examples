/*
题目：古典问题：有一对兔子，从出生后第3个月起每个月都生一对兔子，小兔子长到第三个月后每个月又生一对兔子，假如兔子都不死，问每个月的兔子总数为多少？(对)

程序分析：兔子的规律为数列1,1,2,3,5,8,13,21....
 */

fn rabbit(n: isize) -> isize {
    if n < 3 {
        return 1;
    }

    let mut f1 = 1;
    let mut f2 = 1;
    for _ in 3..=n {
        let f12 = f1 + f2;
        f1 = f2;
        f2 = f12;
    }

    return f2;
}

fn main() {
    println!("Hello, world!");
    for i in 1..20 {
        println!("{} {}", i, rabbit(i));
    }
}
