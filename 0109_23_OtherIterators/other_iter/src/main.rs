
// 调用 map 方法创建一个新迭代器，
// 接着调用 collect 方法消费新迭代器并创建一个 vector
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
