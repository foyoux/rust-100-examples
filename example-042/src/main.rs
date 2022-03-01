/*
题目：学习使用auto定义变量的用法。

Rust 中没有这样的说法，怎么办呢？

发个http请求吧
 */

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);

    return Ok(());
}
