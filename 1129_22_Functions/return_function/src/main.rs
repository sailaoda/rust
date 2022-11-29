fn main() {
    let x = five();
    let y = plus_one(x);
    println!("The value of x is: {x}, y is: {y}");
}

// 单单一个5，没有分号，是一个表达式，可以返回值；
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}