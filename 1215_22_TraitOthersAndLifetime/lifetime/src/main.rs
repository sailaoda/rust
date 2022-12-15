
// 避免悬垂引用
/* fn main() {
    let r;

    {
        let x = 4;
        r = &x;
    } // 出现编译时错误

    println!("r: {}", r);
} */


fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    // 通过拥有不同的具体生命周期的String值调用longest函数
    let string3 = String::from("long string is long");

    {
        let string4 = String::from("xyz");
        let new_result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", new_result);
    }

    // result 引用的生命周期必须是两个参数较小那一个
    /* let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result); */


}

// 返回值需要一个泛型生命周期参数，
// 因为 Rust 并不知道将要返回的引用是指向 x 或 y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 它的实际含义是 longest 函数返回的引用的生命周期
// 与传入该函数的引用的生命周期的较小者一致。
