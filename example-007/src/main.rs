/*
题目：将一个列表的数据复制到另一个列表中。
 */

fn print_list(list: &[i32]) {
    for i in 0..list.len() {
        print!("{} ", list[i]);
    }
    println!();
}

fn main() {
    println!("Hello, world!");
    let list1 = [1, 2, 3];
    print_list(&list1);
    let mut list2 = list1.clone();
    print_list(&list2);

    list2[1] = 222;
    print_list(&list1);
    print_list(&list2);
}
