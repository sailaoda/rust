/*
 * @Author: sailaoda
 * @Date: 2022-12-11 23:36:58
 * @LastEditors: sailaoda
 * @LastEditTime: 2022-12-11 23:52:54
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

    // 只在键没有对应值时插入    entry
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(88);

    println!("{:#?}", scores);

    // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用

    // 根据旧值更新一个新值
    // 通过哈希 map 储存单词和计数来统计出现次数
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // 所有这些改变都是安全的并符合借用规则
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);

    // HashMap 默认使用一种叫做 SipHash 的哈希函数，
    // 它可以抵御涉及哈希表（hash table）1 的拒绝服务（Denial of Service, DoS）攻击。

}
