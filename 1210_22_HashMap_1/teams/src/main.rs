
// 新建一个HashMap 并使用insert增加元素

// 像 vector 一样，哈希 map 将它们的数据储存在堆上，
// 这个 HashMap 的键类型是 String 而值类型是 i32。
// 类似于 vector，哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同类型。

fn main() {

    use std::collections::HashMap;

    // 利用insert 创建HashMap
    let mut scores = HashMap::new;

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 用队伍列表和分数列表创建哈希map
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // 这里 HashMap<_, _> 类型注解是必要的，因为可能 collect 为很多不同的数据结构
    let mut scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();


    // 哈希Map 和所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效

    // 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。
    // 但是这些引用指向的值必须至少在哈希 map 有效时也是有效的。
}