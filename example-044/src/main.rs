/*
两个 3 行 3 列的矩阵，实现其对应位置的数据相加，并返回一个新矩阵：

X = [[12,7,3],
    [4 ,5,6],
    [7 ,8,9]]

Y = [[5,8,1],
    [6,7,3],
    [4,5,9]]
程序分析：创建一个新的 3 行 3 列的矩阵，使用 for 迭代并取出 X 和 Y 矩阵中对应位置的值，相加后放到新矩阵的对应位置中。
 */

fn foo(x: &[[i32; 3]; 3], y: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut z = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            z[i][j] = x[i][j] + y[i][j];
        }
    }
    return z;
}

fn main() {
    println!("Hello, world!");

    let x = [
        [12, 7, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];

    let y = [
        [5, 8, 1],
        [6, 7, 3],
        [4, 5, 9]
    ];

    let z = foo(&x, &y);

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}
