
// 注意 v1_iter 需要是可变的：
// 在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。
// 换句话说，代码 消费（consume）了，或使用了迭代器。
// 每一个 next 调用都会从迭代器中消费一个项。
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    // sum方法使用的测试
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}
