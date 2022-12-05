
// 通过派生trait增加实用功能

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(20 * scale),
        height: 55,
    };

    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels and {}.",
        area(width1, height1),
        area2(&rect1)
    );

    // dbg! 宏接收一个表达式的所有权（与 println! 宏相反，
    // 后者接收的是引用），打印出代码中调用 dbg! 宏时所在的文件和行号，
    // 以及该表达式的结果值，并返回该值的所有权。
    dbg!(&rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// 我们希望借用结构体而不是获取它的所有权，
// 这样 main 函数就可以保持 rect1 的所有权并继续使用它，
// 所以这就是为什么在函数签名和调用的地方会有 &。
fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
