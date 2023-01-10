// 定义 MyBox(x) 类型和 new() 方法
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// `MyBox<T>` 类型不能解引用，因为我们尚未在该类型实现这个功能。
// 为了启用 `*` 运算符的解引用功能，需要实现 `Deref` trait。
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    // deref 方法体中写入了 &self.0，
    // 这样 deref 返回了希望通过 * 运算符访问的值的引用。
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    // Deref 强制转换，使用 MyBox<String> 的引用调用 hello 是可行的
    hello(&m);
    // hello(&(*m)[..]);
    // (*m) 将 MyBox<String> 解引用为 String。
    // 接着 & 和 [..] 获取了整个 String 的字符串 slice 来匹配hello的签名。
}