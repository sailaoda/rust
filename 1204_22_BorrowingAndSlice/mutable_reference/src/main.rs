/*
 * @Author: sailaoda
 * @Date: 2022-12-04 23:08:33
 * @LastEditors: sailaoda
 * @LastEditTime: 2022-12-04 23:12:10
 * @FilePath: \rust\1204_22_BorrowingAndSlice\mutable_reference\src\main.rs
 * @Description:
 *
 * Copyright (c) 2022 by hackerwu.cn, All Rights Reserved.
 */

// 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止

fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}

// 编译器在作用域结束之前判断不再使用的引用的能力被称
// 非词法作用域生命周期（Non-Lexical Lifetimes，简称 NLL）
