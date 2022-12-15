

# Rust

rust daily study

[Rust程序设计语言](https://kaisery.github.io/trpl-zh-cn/)   -->   [Writing an OS in Rust](https://www.bookstack.cn/read/writing-an-os-in-rust/README.md)

-->    [Rust死灵书](https://www.bookstack.cn/read/rustonomicon_zh-CN/src-0.%E7%AE%80%E4%BB%8B.md)



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



## 模块系统

- **包**（*Packages*）： Cargo 的一个功能，它允许你构建、测试和分享 crate。
- **Crates** ：一个模块的树形结构，它形成了库或二进制项目。
- **模块**（*Modules*）和 **use**： 允许你控制作用域和路径的私有性。
- **路径**（*path*）：一个命名例如结构体、函数或模块等项的方式。

crate 是 Rust 在编译时最小的代码单位。

crate 有两种形式：二进制项和库。

允许命名项的 *路径*（*paths*）；用来将路径引入作用域的 `use` 关键字；以及使项变为公有的 `pub` 关键字，还有 `as` 关键字、外部包和 glob 运算符等

- **绝对路径**（*absolute path*）从 crate 根开始，以 crate 名或者字面值 `crate` 开头。
- **相对路径**（*relative path*）从当前模块开始，以 `self`、`super` 或当前模块的标识符开头。

可以使用 `super` 开头来构建从父模块开始的相对路径。这么做类似于文件系统中以 `..` 开头的语法
使用 `as` 关键字提供新的名称  `use std::fmt::Result;   use std::io::Result as IoResult;`

嵌套路径来消除大量的`use`行

```rust
use std::cmp::Ordering;
use std::io;

use std::{self, cmp::Ordering, io};

// glob 运算符
use std::collections::*;
```



## 常见集合

- *vector* 允许我们一个挨着一个地储存一系列数量可变的值
- **字符串**（*string*）是字符的集合。我们之前见过 `String` 类型，不过在本章我们将深入了解。
- **哈希 map**（*hash map*）允许我们将值与一个特定的键（key）相关联。这是一个叫做 *map* 的更通用的数据结构的特定实现。

### Vector

**vector 允许我们在一个单独的数据结构中储存多于一个的值，它在内存中彼此相邻地排列所有的值。vector 只能储存相同类型的值。**

```rust
// 遍历 vector 中的元素
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
    }

    {
        let mut v = vec![100, 23, 43];
        for i in &mut v {
            *i += 50;
        }
    }

v.get(100);
v.push(200);
```

### Strings

```rust
    let s = String::from("initial contents");
    // 等同于
    let s = "initial contents".to_string();

```

使用 push_str 和 push 附加字符串


使用 + 运算符或 format! 宏拼接字符串

**Rust 的字符串不支持索引。**

### HashMap

```rust
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

```



## 错误处理

- ### 用panic!处理不可恢复的错误

当出现 panic 时，程序默认会开始 **展开**（*unwinding*），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。另一种选择是直接 **终止**（*abort*），这会不清理数据就退出程序。那么程序所使用的内存需要由操作系统来清理。

选择 abort 可以直接在 `cargo.toml`文件中的 `[profile]`部分增加 `panic = 'abort'`，可以由展开切换为终止。

**使用 `panic!`的 `backtrace`**

`RUST_BACKTRACE=1 cargo run`

- ### 用Result处理可恢复的错误

#### 使用 match 表达式处理可能会返回的 Result 成员

`Result`枚举定义有两个成员， Ok 和 Err：

```rust
// T 和 E 是泛型类型参数
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`File::open` 函数的返回值类型是 `Result<T, E>`。这里泛型参数 `T` 放入了成功值的类型 `std::fs::File`，它是一个文件句柄。`E` 被用在失败值上时 `E` 的类型是 `std::io::Error`

#### 匹配不同的错误

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:#?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:#?}", other_error)
            }
        },
    };

    println!("Hello, world!");
}
```

#### 失败时panic的简写：`unwrap`和`expect`

```rust
let f = File::open("hello.txt").unwrap();

let f = File::open("hello.txt").expect("Failed to open hello.txt");
```

#### 传播错误

当编写一个其实先会调用一些可能会失败的操作的函数时，除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理。这被称为 **传播**（*propagating*）错误

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

#### 传播错误的简写：`?` 运算符

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    
}
```

`fs::read_to_string` 的函数，它会打开文件、新建一个 `String`、读取文件的内容，并将内容放入 `String`，接着返回它。

`?` 运算符只能被用于返回值与 `?` 作用的值相兼容的函数，?运算符作用于 `File::open`返回的 Result 值。

**只能在返回 `Result` 或者其它实现了 `FromResidual` 的类型的函数中使用 `?` 运算符。**

可以在返回 `Result` 的函数中对 `Result` 使用 `?` 运算符，可以在返回 `Option` 的函数中对 `Option` 使用 `?` 运算符，但是不可以混合搭配。

- ### 要不要panic!

一个 `Guess` 类型，它只在值位于 1 和 100 之间时才继续

```rust
#![allow(unused)]
fn main() {
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
}
```



## 泛型、trait和生命周期

1. 找出重复代码。
2. 将重复代码提取到了一个函数中，并在函数签名中指定了代码中的输入和返回值。
3. 将重复代码的两个实例，改为调用函数。

### 泛型数据结构

我们可以使用泛型为像函数签名或结构体这样的项创建定义，这样它们就可以用于多种不同的具体数据类型。

标准库中定义的 `std::cmp::PartialOrd` trait 可以实现类型的比较功能

#### 结构体中定义的泛型

字段 `x` 和 `y` **都是** 相同类型的

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};
}
```

两个字段有不同类型且仍然是泛型的Point结构体：

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point {x: 5, y: 10};
    let both_float = Point {x: 1.0, y: 4.0};
    let integer_and_float = Point {x: 5, y: 4.0};
}
```

#### 枚举定义中的泛型

```rust
enum Option<T> {
    Some(T),
    None,
}
```

枚举也可以有多个泛型类型， 比如 `Result`

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

#### 方法定义中的泛型

结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型。

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// 在 p1 上以 p2 作为参数调用 mixup 会返回一个 p3，它会有一个 i32 类型的 x，因为 x 来自 p1，并拥有一个 char 类型的 y，因为 y 来自 p2。
fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

#### 泛型代码的性能

Rust 实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失。

Rust 通过在编译时进行泛型代码的 **单态化**（*monomorphization*）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

```

Rust 会为每一个实例编译其特定类型的代码

### Trait：定义共同行为

#### trait 定义

一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话， 这些类型就可以共享相同的行为了。trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。

trait 体中可以有多个方法：一行一个方法签名且都以分号结尾。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

**不能为外部类型实现外部 trait。**

例如，不能在 `aggregator` crate 中为 `Vec<T>` 实现 `Display` trait。这是因为 `Display` 和 `Vec<T>` 都定义于标准库中，它们并不位于 `aggregator` crate 本地作用域中。这个限制是被称为 **相干性**（*coherence*） 的程序属性的一部分，或者更具体的说是 **孤儿规则**（*orphan rule*），其得名于不存在父类型。

impl Trait 是一种较长形式语法的语法糖。

#### trait bound

```rust
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
```



#### 通过 where 简化 trait bound

使用过多的 trait bound 也有缺点。每个泛型有其自己的 trait bound，

所以有多个泛型参数的函数在名称和参数列表之间会有很长的 trait bound 信息，

这使得函数签名难以阅读。

```rust
// 为此，Rust 有另一个在函数签名之后的 where 从句中指定 trait bound 的语法。
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// 使用 where 从句
fn some_function<T, U>(t: &T, u: &U) -> i32
  where T: Display + Clone,
        U: Clone + Debug {}
```

trait 和 trait bound 让我们使用泛型类型参数来减少重复，并仍然能够向编译器明确指定泛型类型需要拥有哪些行为。因为我们向编译器提供了 trait bound 信息，它就可以检查代码中所用到的具体类型是否提供了正确的行为。



### 生命周期确保引用有效

Rust 中的每一个引用都有其 **生命周期**（*lifetime*），也就是引用保持有效的作用域。

#### 生命周期避免了悬垂引用

避免程序引用非预期引用的数据，避免悬垂引用。

当尝试使用离开作用域的值的引用，会出现一个编译时错误。

#### **借用检查器**（*borrow checker*）

Rust 编译器有一个借用检查其，比较作用域来确保所有的借用都是有效的。

```rust
fn main() {
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
}

// 这里 x 拥有生命周期 'b，比 'a 要大。这就意味着 r 可以引用 x：Rust 知道 r 中的引用在 x 有效的时候也总是有效的。
fn main() {
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}
```

数据要有比引用更长的生命周期。

#### 函数中的泛型生命周期

```rust
// 返回值需要一个泛型生命周期参数，
// 因为 Rust 并不知道将要返回的引用是指向 x 或 y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### 生命周期注解语法

生命周期注解描述了多个引用生命周期相互的关系，而不影响其生命周期。

**生命周期参数名称必须以撇号（`'`）开头，其名称通常全是小写，类似于泛型其名称非常短。`'a` 是大多数人默认使用的名称。生命周期参数注解位于引用的 `&` 之后，并有一个空格来将引用类型与生命周期注解分隔开。**

```rust
&i32 		// 引用
&'a i32		// 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
```

单个的生命周期注解本身没有多少意义，因为生命周期注解告诉 Rust 多个引用的泛型生命周期参数如何相互联系的。

#### 函数签名中的生命周期注解

```rust
// longest 函数定义指定了签名中所有的引用必须有相同的生命周期'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

它的实际含义是 longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致。



