/*
 * @Author: sailaoda
 * @Date: 2022-12-11 23:36:58
 * @LastEditors: sailaoda
 * @LastEditTime: 2022-12-11 23:43:56
 * @FilePath: \daily_study\rust\1211_22_HashMap_2\use_hashMap\src\main.rs
 * @Description:
 *
 * Copyright (c) 2022 by hackerwu.cn, All Rights Reserved.
 */
fn main() {

    // get 获得 HashMap 中的值

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // 这里，score 是与蓝队分数相关的值，应为 Some(10)。
    // 因为 get 返回 Option<V>，所以结果被装进 Some；
    // 如果某个键在哈希 map 中没有对应的值，get 会返回 None。

    // 可以使用与 vector 类似的方式来遍历哈希 map 中的每一个键值对，也就是 for 循环：
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新HashMap
    // 如果我们插入了一个键值对，接着用相同的键插入一个不同的值，与这个键相关联的旧值将被替换。

    scores.insert(String::from("Blue"), 30);
    println!("{:#?}", scores);

    
}
