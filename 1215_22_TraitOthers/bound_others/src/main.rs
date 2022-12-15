
// 使用 trait bound 有条件地实现方法

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

// 在下一个 impl 块中，只有那些为 T 类型实现了 PartialOrd trait （来允许比较）
// 和 Display trait （来启用打印）的 Pair<T>
// 才会实现 cmp_display 方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 可以对任何实现了特定 trait 的类型有条件地实现 trait。
// 对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations

// eg: 标准库为任何实现了 Display trait 的类型实现了 ToString trait
impl<T: Display> ToString for T {
    // snip
}
// 因此，我们可以对任何实现了 Display trait 的类型调用
// 由 ToString 定义的 to_string 方法
let s = 2.to_string();


