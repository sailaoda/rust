// 结构体 CustomSmartPointer，其实现了放置清理代码的 Drop trait
struct CustomSmartPointer {
    data: String,
}

// drop 函数体是放置任何当类型实例离开作用域时期望运行的逻辑的地方。
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// Drop trait 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。
// 为了能够看出 Rust 何时调用 drop，让我们暂时使用 println! 语句实现 drop。
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // 尝试手动调用 Drop trait 的 drop 方法提早清理
    let e = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // 报错 不允许显式调用drop
    // e.drop();
    drop(e);
    println!("CustomSmartPointer dropped before the end of main.");
}


