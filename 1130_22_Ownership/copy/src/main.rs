/*
 * @Author: sailaoda
 * @Date: 2022-11-30 23:36:57
 * @LastEditors: sailaoda
 * @LastEditTime: 2022-11-30 23:39:26
 * @FilePath: \rust\1130_22_Ownership\copy\src\main.rs
 * @Description:
 *
 * Copyright (c) 2022 by hackerwu.cn, All Rights Reserved.
 */
fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
