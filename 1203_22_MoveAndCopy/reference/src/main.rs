// 引用reference像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于
// 其他变量的数据。
// 与指针不同，引用确保指向某个特定类型的有效值

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// 符号&就是引用，它们允许使用值但不获取其所有权
// s 只有指针ptr，引用的s1 的ptr


// 与使用&引用相反的操作是  解引用 dereferencing， 它使用解引用运算符 *