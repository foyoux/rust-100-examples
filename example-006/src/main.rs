/*
题目：斐波那契数列。

程序分析：斐波那契数列（Fibonacci sequence），又称黄金分割数列，指的是这样一个数列：0、1、1、2、3、5、8、13、21、34、……。

在数学上，费波那契数列是以递归的方法来定义：

F0 = 0     (n=0)
F1 = 1    (n=1)
Fn = F[n-1]+ F[n-2](n=>2)
 */

fn fibonacci1(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    return fibonacci1(n - 1) + fibonacci1(n - 2);
}

fn fibonacci2(n: u16) -> u128 {
    if n < 2 {
        return u128::from(n);
    }

    let mut f1 = 0;
    let mut f2 = 1;
    for _ in 2..=n {
        let f22 = f1 + f2;
        f1 = f2;
        f2 = f22;
    }

    return f2;
}

fn main() {
    println!("Hello, world!");
    for i in 0..=100 {
        println!("{} {}", i, fibonacci2(i));
    }
}
