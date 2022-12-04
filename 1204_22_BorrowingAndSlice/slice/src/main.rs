
// first_word 函数返回 String 参数的一个字节索引值
fn first_word(s: &String) -> usize {
    // 需要逐个检查是否有空格， 需要用 as_bytes 方法将 String 转化为字节数组
    let bytes = s.as_bytes();

    // 用 iter 方法在字节数组上创建一个迭代器
    // iter 方法返回集合中的每一个元素

    // enumerate 包装了iter的结果，将这些元素作为元组的一部分来返回。
    // enumerate 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用。这比我们自己计算索引要方便一些

    // enumerate 方法返回一个元组，我们可以使用模式来解构。
    // 在for循环中， 我们指定了一个模式，其中元组中的i是索引而元组中的&item是单个字节
    // 我们从.iter().enumerate()中获取了集合元素的引用，所以模式中使用了&
    for (i, &item) in bytes.iter().enumerate() {

        // 在for循环中，我们通过字节的字面值语法来寻找代表空格的字节。
        // 如果找到了一个空格，返回它的位置。
        if item == b' ' {
            return i;
        }
    }

    // 否则，使用s.len()返回字符串的长度
    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    // starting_index 是 slice 的第一个位置，ending_index 则是 slice 最后一个位置的后一个值

    // 如果想要从索引 0 开始，可以不写两个点号之前的值。
    // 如果 slice 包含 String 的最后一个字节，也可以舍弃尾部的数字
    let hello1 = &s[0..5];
    let hello2 = &s[..5];
    let world1 = &s[6..11];
    let world2 = &s[6..];
    println!("{}, {}", hello2, world2);

    let word = first_word(&s);
    println!("{}", word);

    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！


}

/* let s = "Hello, world!";
这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。 */


/* 所有权、借用和 slice 这些概念让 Rust 程序在编译时确保内存安全。
Rust 语言提供了跟其他系统编程语言相同的方式来控制你使用的内存，
但拥有数据所有者在离开作用域后自动清除其数据的功能意味着你无须额外编写和调试相关的控制代码。 */
