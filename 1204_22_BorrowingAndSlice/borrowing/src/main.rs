/*
 * @Author: sailaoda
 * @Date: 2022-12-03 14:28:45
 * @LastEditors: sailaoda
 * @LastEditTime: 2022-12-04 23:07:45
 * @FilePath: \rust\1204_22_BorrowingAndSlice\borrowing\src\main.rs
 * @Description:
 *
 * Copyright (c) 2022 by hackerwu.cn, All Rights Reserved.
 */

// 我们将创建一个引用的行为称为借用（borrowing）
// 正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值

// 可变引用
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}, {}, {}", s, &s, s.len())
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。


/* Rust 可以在编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，它可由这三个行为造成：

两个或更多指针同时访问同一数据。
至少有一个指针被用来写入数据。
没有同步数据访问的机制。 */