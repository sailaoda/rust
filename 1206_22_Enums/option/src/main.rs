/*
 * @Author: sailaoda
 * @Date: 2022-12-06 00:10:17
 * @LastEditors: sailaoda
 * @LastEditTime: 2022-12-06 00:22:29
 * @FilePath: \daily_study\rust\1206_22_Enums\option\src\main.rs
 * @Description:
 *
 * Copyright (c) 2022 by hackerwu.cn, All Rights Reserved.
 */
enum Option<T> {
    None,
    Some<T>,
}

let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;


let x: i8 = 5;
let y: Option<i8> = Some(5);
// 报错，类型不同
let sum = x + y;