// Rust 的核心语言中只有一种字符串类型：字符串slice str，它通常以被借用的形式出现，&str
// 称作 String 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。

fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // 等同于  String::from("initial contents")
    let s = "initial contents".to_string();

    // 字符串是 UTF-8 编码的，所以可以包含任何可以正确编码的数据

    let mut s = String::from("foo");
    s.push_str("bar");

    // 这里不会获得 s2的所有权
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // &String 可以被 强转（coerced）成 &str

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    // 遍历字符串
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    println!("Hello, world!");
}
