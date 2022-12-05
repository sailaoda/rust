/*
 * @Author: sailaoda
 * @Date: 2022-12-05 10:31:46
 * @LastEditors: sailaoda
 * @LastEditTime: 2022-12-05 11:14:35
 * @FilePath: \rust\1205_22_Structure\defining_structs\src\main.rs
 * @Description:
 *
 * Copyright (c) 2022 by hackerwu.cn, All Rights Reserved.
 */

// 在大括号中，定义每一部分数据的名字和类型，称为字段field

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@qq.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@qq.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@qq.com"),
        sign_in_count: user1.sign_in_count,
    };

    // .. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值
    let user3 = User {
        email: String::from("another@qq.com"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 使用结构体更新语法  struct update syntax
// 结构更新语法就像带有 = 的赋值，因为它移动了数据


// 元组结构体（tuple structs）。元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。