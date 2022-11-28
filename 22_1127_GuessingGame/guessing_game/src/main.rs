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
