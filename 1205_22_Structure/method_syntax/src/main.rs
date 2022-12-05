/*
 * @Author: sailaoda
 * @Date: 2022-12-05 12:29:21
 * @LastEditors: sailaoda
 * @LastEditTime: 2022-12-05 12:58:59
 * @FilePath: \rust\1205_22_Structure\method_syntax\src\main.rs
 * @Description:
 *
 * Copyright (c) 2022 by hackerwu.cn, All Rights Reserved.
 */

// 方法method与函数类似：
// 他们使用fn关键字和名称声明，可以拥有参数和返回值，
// 同时包含在某处调用该方法时会执行的代码。

// 不同点是他们在结构体的上下文中被定义（或者是枚举或trait对象的上下文）
// 它们第一个参数总是self，它代表调用该方法的结构体实例

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 是 implementation 的缩写
impl Rectangle {
    //选用 &self 是因为 不想获得所有权，只希望能够读取结构体中的数据，而不是写入
    // 如果想要在方法中改变调用方法的实例，
    // 需要将第一个参数改为&mut self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 在方法签名中，可以在self后增加多个参数，
    // 这些参数就像函数中的参数一样工作
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 40,
        height: 43,
    };

    let rect2 = Rectangle {
        width: 22,
        height: 33,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let sq = Rectangle::square(3);

    dbg!(&sq);
}

// Rust 有一个叫自动引用和解引用 automatic referencing and dereferencing
// 方法调用是Rust拥有这种行为的地方

// 即 以下代码是等价的：
// p1.distance(&p2);
// (&p1).distance(&p2);

// 关联函数：所有在impl块中定义的函数被称为关联函数 associated functions
// 他们与impl后面命名的类型相关
// 可以定义不以self为第一参数的关联函数（因此不是方法）

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
//关键字 Self 在函数的返回类型中代指在 impl 关键字后出现的类型，
//在这里是 Rectangle

// 使用结构体名和 :: 语法来调用这个关联函数：
// 比如 let sq = Rectangle::square(3);。这个函数位于结构体的命名空间中：
// :: 语法用于关联函数和模块创建的命名空间。


// 每个结构体都允许拥有多个impl块