

# Rust

rust daily study

// 杭州早上七点十分左右 Github 进入下一天

## 入门

- 使用 `rustup` 安装最新稳定版的 Rust
- 更新到新版的 Rust
- 打开本地安装的文档
- 直接通过 `rustc` 编写并运行 Hello, world! 程序
- 使用 Cargo 创建并运行新项目

## 常见概念

变量、标量和复合数据类型、函数、注释、 `if` 表达式和循环

## 所有权

- 在任意给定时间，**要么** 只能有一个可变引用，**要么** 只能有多个不可变引用。
- 引用必须总是有效的。

所有权、借用和 slice 这些概念让 Rust 程序在编译时确保内存安全。Rust 语言提供了跟其他系统编程语言相同的方式来控制你使用的内存，但拥有数据所有者在离开作用域后自动清除其数据的功能意味着你无须额外编写和调试相关的控制代码。

## 结构体

结构体让你可以创建出在你的领域中有意义的自定义类型。通过结构体，我们可以将相关联的数据片段联系起来并命名它们，这样可以使得代码更加清晰。在 `impl` 块中，你可以定义与你的类型相关联的函数，而方法是一种相关联的函数，让你指定结构体的实例所具有的行为。

- 结构体
- 元组结构体
- 结构体方法
- 结构体关联函数

## 枚举和模式匹配

### 枚举

- 每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数
- 用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据
- 枚举也可以定义方法，和使用`impl`为结构体定义方法一样`（&self）`

### Option

- Option枚举和其相对于空值的优势,限制空值的泛滥以增加Rust代码的安全性

- Rust并没有空值，不过可以拥有一个可以编码存在或不存在概念的枚举

- ```rust
  enum Option<T> {
      None,
      Some(T),
  }
  ```

- `Option<T>` 和`T`是不同的类型，例如 `Option<i8>` 和 `i8`

### match

`match` 的极为强大的控制流运算符，它允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码

**匹配是穷尽的**，这些分支必须覆盖了所有的可能性。

### 通配模式和_占位符

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    // other => move_player(other),
    // _ => reroll(),
    _ => (),    // 不运行任何代码
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
```

使用 `_` ，这是一个特殊的模式，可以匹配任意值而不绑定到该值。这告诉 Rust 我们不会使用这个值，所以 Rust 也不会警告我们存在未使用的变量。

### if let 简洁控制流

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

等价于

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

