// 使用match将错误返回给代码调用者
use std::fs::{self, File};
use std::io::{self, Read};

fn read_username_from_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s);
    Ok(s)
}

// 可以在 ? 之后直接使用链式方法调用来进一步缩短代码
fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}


// 从给定文本中返回第一行最后一个字符的函数的例子
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

use std::error::Error;

// Box<dyn Error> 类型是一个 trait 对象（trait object
// 目前可以将 Box<dyn Error> 理解为 “任何类型的错误”。
// 在返回 Box<dyn Error> 错误类型 main 函数中对 Result 使用 ? 是允许的，
// 因为它允许任何 Err 值提前返回。
fn main() -> Result<(), Box<dyn Error>> {
    let mut f = last_char_of_first_line("wouiyigedauyge");
    println!("{:?}", f);

    Ok(())
}

// 当 main 函数返回 Result<(), E>，如果 main 返回 Ok(()) 可执行程序会以 0 值退出，而如果 main 返回 Err 值则会以非零值退出；
// 成功退出的程序会返回整数 0，运行错误的程序会返回非 0 的整数。