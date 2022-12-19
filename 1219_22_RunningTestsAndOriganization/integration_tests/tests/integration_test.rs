// 一个 integration_tests crate 中函数的集成测试

use integration_tests;

mod common_new;

#[test]
fn it_adds_two() {
    common_new::setup();
    assert_eq!(4, integration_tests::add_two(2));
}