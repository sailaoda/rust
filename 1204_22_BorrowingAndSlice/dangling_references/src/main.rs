// 在具有指针的语言中，很容易通过释放内存是保留指向它的指针而错误地生成一个
// 悬垂指针。

// 所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    // &s
    s
}


// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效的。