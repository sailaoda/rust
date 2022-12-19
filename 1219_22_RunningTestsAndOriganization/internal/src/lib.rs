pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
// 注意 internal_adder 函数并没有标记为 pub。
// 子模块的项可以使用其上级模块的项。
// 在测试中，我们通过 use super::* 将 test 模块的父模块的所有项引入了作用域，
// 接着测试调用了 internal_adder。