/*
题目：对10个数进行排序。

程序分析：可以利用选择法，即从后9个比较过程中，选择一个最小的与第一个元素交换，下次类推，即用第二个元素与后8个进行比较，并进行交换。
 */

fn main() {
    println!("Hello, world!");

    let mut nums1 = [23, 432, 54, 23, 54342, 56, 443, 5, 332, 12];
    nums1.sort0();
    println!("{:?}", nums1);

    let mut nums1 = [23, 432, 54, 23, 54342, 56, 443, 5, 332, 12];
    nums1.sort1();
    println!("{:?}", nums1);
}

trait Sort {
    fn sort0(&mut self);
    fn sort1(&mut self);
}

impl Sort for [i32] {
    fn sort0(&mut self) {
        self.sort();
    }

    fn sort1(&mut self) {
        // 冒泡排序
        for j in (2..self.len()).rev() {
            for i in 1..=j {
                if self[i - 1] > self[i] {
                    // 交换值
                    // let tmp = self[i];
                    // self[i] = self[i - 1];
                    // self[i - 1] = tmp;
                    self[i] = self[i] ^ self[i - 1];
                    self[i - 1] = self[i] ^ self[i - 1];
                    self[i] = self[i] ^ self[i - 1];
                }
            }
        }
    }
}


