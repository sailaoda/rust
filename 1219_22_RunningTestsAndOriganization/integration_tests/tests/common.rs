pub fn setup() {
    // setup code specific to your library's test would go here
}


// 为了不让 common 出现在测试输出中，我们将创建 tests/common/mod.rs ，
// 而不是创建 tests/common.rs 。这是一种 Rust 的命名规范，
// 这样命名告诉 Rust 不要将 common 看作一个集成测试文件。