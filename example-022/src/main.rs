/*
题目：两个乒乓球队进行比赛，各出三人。甲队为a,b,c三人，乙队为x,y,z三人。已抽签决定比赛名单。有人向队员打听比赛的名单。a说他不和x比，c说他不和x,z比，请编程序找出三队赛手的名单。
 */
fn main() {
    println!("Hello, world!");
    // 代码未优化
    for a in 'x'..='z' {
        for b in 'x'..='z' {
            for c in 'x'..='z' {
                // println!("a {}, b {}, c {}", a, b, c);
                if a != b && a != c && b != c {
                    if a != 'x' && c != 'x' && c != 'z' {
                        println!("a -> {}, b -> {}, c -> {}", a, b, c);
                    }
                }
            }
        }
    }
}
