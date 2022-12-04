/*
 * @Author: sailaoda
 * @Date: 2022-12-03 07:46:00
 * @LastEditors: sailaoda
 * @LastEditTime: 2022-12-04 23:27:56
 * @FilePath: \rust\1203_22_MoveAndCopy\about_move\src\main.rs
 * @Description:
 *
 * Copyright (c) 2022 by hackerwu.cn, All Rights Reserved.
 */
fn main() {
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}");
    // 为了避免 } 时二次释放(double free)：离开作用域时，他们都会尝试释放相同的内存。

    // 为了保证内存安全， 即  let s1 = s2 后，Rust认为 s1 不再有效。此后再引用s1会被禁止使用

    // s2 拷贝了s1 的指针ptr、长度len和容量capacity，然后使第一个变量无效

    // Rust永远也不会自动创建数据的“深拷贝”，任何自动的复制都可以被认为运行时性能影响最小
}
