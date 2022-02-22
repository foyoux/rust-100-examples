/*
题目：有四个数字：1、2、3、4，能组成多少个互不相同且无重复数字的三位数？各是多少？

程序分析：可填在百位、十位、个位的数字都是1、2、3、4。组成所有的排列后再去 掉不满足条件的排列。

*/


fn main() {
    let arr = [1, 2, 3, 4];
    let mut count: u32 = 0;
    for i in arr {
        for j in arr {
            for k in arr {
                if i != j && i != k && j != k {
                    println!("{} {} {}", i, j, k);
                    count += 1;
                }
            }
        }
    }
    println!("共 {} 种", count);
}


