fn main() {
    println!("Hello, world!");
}

// trait bound
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// impl Trait 很方便，适用于短小的例子。trait bound 则适用于更复杂的场景。

pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

// item1 和 item2 需要时相同类型时
pub fn notify<T: Summary>(item1: &T, item2: &T) {}


// 通过 + 指定多个 trait bound
pub fn notify(item: &(impl Summary + Display)) {}

pub fn notify<T:Summary + Display>(item: &T) {}


// 通过 where 简化 trait bound
//使用过多的 trait bound 也有缺点。每个泛型有其自己的 trait bound，
// 所以有多个泛型参数的函数在名称和参数列表之间会有很长的 trait bound 信息，
// 这使得函数签名难以阅读。
// 为此，Rust 有另一个在函数签名之后的 where 从句中指定 trait bound 的语法。
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// 使用 where 从句
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {}

