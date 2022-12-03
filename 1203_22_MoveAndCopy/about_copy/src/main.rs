// 克隆
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);


    // 只在栈上的数据
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s3 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

// 任何不需要分配内存或某种形式资源的类型都可以实现Copy。 如下是一些Copy的类型：
/* 所有的整数类型，比如u32。
布尔类型， bool， 它的值是true和false
所有浮点数类型， 比如f64
字符类型， char
元组，当且仅当其包含的类型也都实现Copy的时候， 比如， (i32, i32) 实现了Copy， 但(i32, String)就没有
 */