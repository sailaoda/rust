# Rust猜数字游戏

```rust
let guess: u32 = guess.trim().parse.expect("Please type a number");
```

`trim()` 去除字符串中的空格和换行符等  `\r 、\n`

字符串的`parse()`方法将字符串转换成其他类型。 guess后面的冒号(:)告诉Rust指定的变量的类型。



将`expect`调用换成`match`语句， 以从遇到错误就崩溃转换为处理错误。  `parse`返回一个`Result`类型， 是一个拥有`Ok`或`Err`成员的枚举。  这里使用的`match`表达式，和之前处理`cmp`方法返回`Ordering`时用的一样。

`Err(_)` 中的 `_`是一个通配符值， 本例中用来匹配所有Err值，执行第二个分支的动作。



```rust
/*
 * @Author: sailaoda
 * @Date: 2022-11-24 19:25:55
 * @LastEditors: sailaoda
 * @LastEditTime: 2022-11-28 09:24:06
 * @FilePath: \guessing_game\src\main.rs
 * @Description:
 *
 * Copyright (c) 2022 by hackerwu.cn, All Rights Reserved.
 */

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("let`s go!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

```

