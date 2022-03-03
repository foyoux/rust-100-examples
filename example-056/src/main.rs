/*
题目：画图，学用circle画圆形。

用 rui 画图
 */
use rui::*;

mod rui;

fn main() {
    rui(hstack! {
        circle()
            .color(RED_HIGHLIGHT)
            .padding(Auto)
    });
}
