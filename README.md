

# Rust

rust daily study

[Rust程序设计语言](https://kaisery.github.io/trpl-zh-cn/)   -->      [Rust死灵书](https://www.bookstack.cn/read/rustonomicon_zh-CN/src-0.%E7%AE%80%E4%BB%8B.md)



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

HashMap 的几种操作

```rust
map.insert();

map.entry().or_insert();

for (key, value) in &map {
    println!("{}: {}", key, value);
}
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

一个存放引用的结构体，其定义需要生命周期注解

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

#### 生命周期省略（Lifetime Elision）

被编码进 Rust 引用分析的模式被称为 **生命周期省略规则**（*lifetime elision rules*）。这并不是需要程序员遵守的规则；这些规则是一系列特定的场景，此时编译器会考虑，如果代码符合这些场景，就无需明确指定生命周期。

函数或方法的参数的生命周期被称为 **输入生命周期**（*input lifetimes*），而返回值的生命周期被称为 **输出生命周期**（*output lifetimes*）。

编译器采用三条规则来判断引用何时不需要明确的注解。第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。

第一条规则是每一个是引用的参数都有它自己的生命周期参数。

第二条规则是如果只有一个输入生命周期参数

第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 `&self` 或 `&mut self`，说明是个对象的方法(method)(译者注： 这里涉及rust的面向对象参见17章)，那么所有输出生命周期参数被赋予 `self` 的生命周期。

#### 方法定义中的生命周期注解

（实现方法时）结构体字段的生命周期必须总是在 `impl` 关键字之后声明并在结构体名称之后被使用，因为这些生命周期是结构体类型的一部分。

`impl` 块里的方法签名中，引用可能与结构体字段中的引用相关联，也可能是独立的。另外，生命周期省略规则也经常让我们无需在方法签名中使用生命周期注解。

#### 静态生命周期

`'static` 其生命周期能够存活于整个程序期间。所有的字符串字面值都拥有 `'static`生命周期

```rust
let s: &'static str = "I have a static lifetime.";
```

这个字符串的文本被直接储存在程序的二进制文件中而这个文件总是可用的。因此所有的字符串字面值都是 `'static` 的。

考虑一下再使用，是否真的要让它的生命周期存在得那么久。

#### 结合泛型类型参数、trait bounds 和生命周期

```rust
// 在同一函数中指定泛型类型参数、trait bounds 和生命周期的语法

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

```



## 编写自动化测试

Rust 中的测试函数是用来验证非测试代码是否按照期望的方式运行的。测试函数体通常执行如下三种操作：

1. 设置任何所需的数据或状态
2. 运行需要测试的代码
3. 断言其结果是我们所期望的

### 编写测试

#### 测试函数

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

#### 使用 assert! 宏来检查结果

`assert!` 宏由标准库提供，在希望确保测试中一些条件为 `true` 时非常有用。需要向 `assert!` 宏提供一个求值为布尔值的参数。如果值是 `true`，`assert!` 什么也不做，同时测试会通过。如果值为 `false`，`assert!` 调用 `panic!` 宏，这会导致测试失败。

```rust
#[cfg(test)]
mod tests {
	// 这是一个内部模块，要测试外部模块中的代码，
    // 需要将其引入到内部模块的作用域中。这里选择使用 glob 全局导入，
    // 以便在 tests 模块中使用所有在外部模块定义的内容。
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

```

#### 使用 assert_eq! （相等）和 assert_ne! （不相等）宏来测试相等

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

#### 自定义失败信息

可以向 `assert!`、`assert_eq!` 和 `assert_ne!` 宏传递一个可选的失败信息参数，可以在测试失败时将自定义失败信息一同打印出来。

```rust
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
```

#### 使用`should_panic`检查panic

#### 将 Result<T, E> 用于测试

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

不能对这些使用 `Result<T, E>` 的测试使用 `#[should_panic]` 注解。为了断言一个操作返回 `Err` 成员，**不要**使用对 `Result<T, E>` 值使用问号表达式（`?`）。而是使用 `assert!(value.is_err())`。



### 控制测试如何运行

可以指定命令行参数来改变 `cargo test` 的默认行为。

可以将一部分命令行参数传递给 `cargo test`，而将另外一部分传递给生成的测试二进制文件。为了分隔这两种参数，需要先列出传递给 `cargo test` 的参数，接着是分隔符 `--`，再之后是传递给测试二进制文件的参数。

#### 并行或连续的运行测试

当运行多个测试时， Rust 默认使用线程来并行运行。

因为测试是在同时运行的，你应该确保测试不能相互依赖，或依赖任何共享的状态，包括依赖共享的环境，比如当前工作目录或者环境变量。

如果你不希望测试并行运行，或者想要更加精确的控制线程的数量，可以传递 `--test-threads` 参数和希望使用线程的数量给测试二进制文件。例如：

```bash
cargo test -- --test-threads=1
```

这里将测试线程设置为 `1`，告诉程序不要使用任何并行机制。

#### 显示函数输出

默认情况下，当测试通过时，Rust的测试库会截获打印到标准输出的所有内容。

如下，一个调用了`println!`的函数的测试，

它打印参数的值并接着返回10，接着还有一个会通过的测试和一个会失败的测试：

```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    
    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

如果你希望也能看到通过的测试中打印的值，也可以在结尾加上 `--show-output` 告诉 Rust 显示成功测试的输出。

```bash
cargo test -- --show-output
```

#### 通过指定名字来运行部分测试

可以向 `cargo test` 传递所希望运行的测试名称的参数来选择运行哪些测试。

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }
    
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

运行全部测试：`cargo test`

运行单个测试：`cargo test one_hundred`

可以向`cargo test`传递任意测试的名称来只运行这个测试。只有传递给 `cargo test` 的第一个值才会被使用。

#### 过滤运行多个测试

我们可以指定部分测试的名称，任何名称匹配这个名称的测试会被运行。

例如：`cargo test add`， 可以只运行那两个带有add的测试函数。同时注意测试所在的模块也是测试名称的一部分，所以可以通过模块名来运行一个模块中的所有测试。

#### 忽略某些测试

使用 `ignore` 属性来标记耗时的测试并排除他们。

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 需要运行一个小时的代码
}
```

对于想要排除的测试，我们在 `#[test]` 之后增加了 `#[ignore]` 行。

如果我们只希望运行被忽略的测试，可以使用 `cargo test -- --ignored`

不管是否忽略都要运行全部测试，可以运行 `cargo test -- --include-ignored`。



### 测试的组织架构

根据测试的两个主要分类来考虑问题：**单元测试**（*unit tests*）与 **集成测试**（*integration tests*）。

单元测试倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口。而集成测试对于你的库来说则完全是外部的。它们与其他外部代码一样，通过相同的方式使用你的代码，只测试公有接口而且每个测试都有可能会测试多个模块。

#### 单元测试

单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码，以便于快速而准确的某个单元的代码功能是否符合预期。单元测试与他们要测试的代码共同存放在位于src目录下相同的文件中。规范是在每个文件中创建包含测试函数的tests模块，并使用cfg(test)标注模块。

##### 测试模块和#[cfg(test)]

测试模块的 `#[cfg(test)]` 注解告诉 Rust 只在执行 `cargo test` 时才编译和运行测试代码。

与之对应的集成测试因为位于另一个文件夹，所以它们并不需要 `#[cfg(test)]` 注解。然而单元测试位于与源码相同的文件中，所以你需要使用 `#[cfg(test)]` 来指定他们不应该被包含进编译结果中。

`cfg` 属性代表 *configuration* ，它告诉 Rust 其之后的项只应该被包含进特定配置选项中。

##### 测试私有函数

Rust 的私有性规则确实允许你测试私有函数。

```rust
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
```

注意 internal_adder 函数并没有标记为 pub。

子模块的项可以使用其上级模块的项。

在测试中，我们通过 use super::* 将 test 模块的父模块的所有项引入了作用域，接着测试调用了 internal_adder。

#### 集成测试

在 Rust 中，集成测试对于你需要测试的库来说完全是外部的。同其他使用库的代码一样使用库文件，也就是说它们只能调用一部分库中的公有 API 。集成测试的目的是测试库的多个部分能否一起正常工作。一些单独能正确运行的代码单元集成在一起也可能会出现问题，所以集成测试的覆盖率也是很重要的。为了创建集成测试，你需要先创建一个 *tests* 目录。

##### tests 目录

为了编写集成测试，需要在项目根目录创建一个 *tests* 目录，与 *src* 同级。

接着可以随意在这个目录中创建任意多的测试文件，Cargo 会将每一个文件当作单独的 crate 来编译。

与单元测试不同，我们需要在文件顶部添加 `use adder`。这是因为每一个 `tests` 目录中的测试文件都是完全独立的 crate，所以需要在每一个文件中导入库。

```rust
use integration_tests;

#[test]
fn it_adds_two() {
    assert_eq!(4, integration_tests::add_two(2));
}
```

现在有了三个部分的输出：单元测试、集成测试和文档测试。

我们仍然可以通过指定测试函数的名称作为 `cargo test` 的参数来运行特定集成测试。也可以使用 `cargo test` 的 `--test` 后跟文件的名称来运行某个特定集成测试文件中的所有测试：

```bash
cargo test --test integration_test
```

这个命令只运行了 *tests* 目录中我们指定的文件 `integration_test.rs` 中的测试。

##### 集成测试中的子模块

将每个集成测试文件当作其自己的 crate 来对待，这更有助于创建单独的作用域，这种单独的作用域能提供更类似与最终使用者使用 crate 的环境。

##### 二进制crate的集成测试

如果项目是二进制 crate 并且只包含 *src/main.rs* 而没有 *src/lib.rs*，这样就不可能在 *tests* 目录创建集成测试并使用 `extern crate` 导入 *src/main.rs* 中定义的函数。只有库 crate 才会向其他 crate 暴露了可供调用和使用的函数；二进制 crate 只意在单独运行。

这就是许多 Rust 二进制项目使用一个简单的 *src/main.rs* 调用 *src/lib.rs* 中的逻辑的原因之一。因为通过这种结构，集成测试 **就可以** 通过 `extern crate` 测试库 crate 中的主要功能了，而如果这些重要的功能没有问题的话，*src/main.rs* 中的少量代码也就会正常工作且不需要测试。



## 一个I/O项目：构建一个命令行程序

我们将创建一个我们自己版本的经典命令行工具： `grep`。 `grep`是"Globally search a Regular Expression and Print." 的首字母缩写。`grep`最简单的使用场景是在特定文件中搜索指定字符串。为此，`grep`获取一个文件名和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行。

### 接受命令行参数

能够获取传递给它的命令行参数的值：Rust标准库提供的函数，`std::env::args`。这个函数返回一个传递给程序的命令行参数的迭代器(iterator)。

迭代器的两个细节：迭代器生成一系列的值，可以在迭代器上调用 collect 方法将其转换为一个集合。

### 重构改进模块性和错误处理

`parse_config` 名称的 `config` 部分，它暗示了我们返回的两个值是相关的并都是一个配置值的一部分。目前除了将这两个值组合进元组之外并没有表达这个数据结构的意义：我们可以将这两个值放入一个结构体并给每个字段一个有意义的名字。这会让未来的维护者更容易理解不同的值如何相互关联以及他们的目的。

这种在复杂类型更为合适的场景下使用基本类型的反模式称为 **基本类型偏执**（*primitive obsession*）

```rust
use std::env;
use std::fs;

/* 注意 std::env::args 在其任何参数包含无效 Unicode 字符时会 panic。
如果你需要接受包含无效 Unicode 字符的参数，使用 std::env::args_os 代替。
这个函数返回 OsString 值而不是 String 值。
这里出于简单考虑使用了 std::env::args，
因为 OsString 值每个平台都不一样而且比 String 值处理起来更为复杂。 */
fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let filename = &args[2];
    // let (query, filename) = parse_config(&args);
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// 组合配置值
struct Config {
    query: String,
    filename: String,
}

// 提取参数解析器
/* fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
} */
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {query, filename}
}
```

### 处理环境变量

我们将增加一个额外的功能来改进 `minigrep`：用户可以通过设置环境变量来设置搜索是否是大小写敏感的 。

最后需要实际检查环境变量。处理环境变量的函数位于标准库的 `env` 模块中，所以我们需要在 *src/lib.rs* 的开头增加一个 `use std::env;` 行将这个模块引入作用域中。接着在 `Config::new` 中使用 `env` 模块的 `var` 方法来检查一个叫做 `CASE_INSENSITIVE` 的环境变量

```rust
use std::env;
// --snip--

use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
```

一些程序允许对相同配置同时使用参数 **和** 环境变量。在这种情况下，程序来决定参数和环境变量的优先级。作为一个留给你的测试，尝试通过一个命令行参数或一个环境变量来控制大小写不敏感搜索。并在运行程序时遇到矛盾值时决定命令行参数和环境变量的优先级。



## Rust中的函数式语言功能：迭代器与闭包

- **闭包**（*Closures*），一个可以储存在变量里的类似函数的结构
- **迭代器**（*Iterators*），一种处理元素序列的方式
- 如何使用这些功能来改进第十二章的 I/O 项目。
- 这两个功能的性能。（**剧透警告：** 他们的速度超乎你的想象！）

### 闭包：可以捕获其环境的匿名函数

#### 重构使用闭包存储代码

```rust
// 重构使用闭包储存代码
use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
```

闭包定义是 `expensive_closure` 赋值的 `=` 之后的部分。闭包的定义以一对竖线（`|`）开始，在竖线中指定闭包的参数；之所以选择这个语法是因为它与 Smalltalk 和 Ruby 的闭包定义类似。这个闭包有一个参数 `num`；如果有多于一个参数，可以使用逗号分隔，比如 `|param1, param2|`。

参数之后是存放闭包体的大括号 —— 如果闭包体只有一行则大括号是可以省略的。大括号之后闭包的结尾，需要用于 `let` 语句的分号。因为闭包体的最后一行没有分号（正如函数体一样），所以闭包体（`num`）最后一行的返回值作为调用闭包时的返回值 。

#### 闭包类型推断和注解

```rust
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

没有加的对比

```rust
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

闭包语法，有类型注解闭包的语法较为类似函数：

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

**闭包定义会为每个参数和返回值推断一个具体类型。**

```rust
fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
}
// 尝试调用一个被推断为两个不同类型的闭包,会报错
```

第一次使用 `String` 值调用 `example_closure` 时，编译器推断 `x` 和此闭包返回值的类型为 `String`。接着这些类型被锁定进闭包 `example_closure` 中，如果尝试对同一闭包使用不同类型则会得到类型错误。

#### 使用带有泛型和Fn trait的闭包

可以创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。你可能见过这种模式被称 *memoization* 或 *lazy evaluation* *（惰性求值）*。

为了让结构体存放闭包，我们需要指定闭包的类型，因为结构体定义需要知道其每一个字段的类型。每一个闭包实例有其自己独有的匿名类型：也就是说，即便两个闭包有着相同的签名，他们的类型仍然可以被认为是不同。为了定义使用闭包的结构体、枚举或函数参数，需要使用泛型和 trait bound。

`Fn` 系列 trait 由标准库提供。所有的闭包都实现了 trait `Fn`、`FnMut` 或 `FnOnce` 中的一个。

存放了闭包和一个 Option 结果值的 `Cacher` 结构体的定义：

```rust
// 定义一个 Cacher 结构体来在 calculation 中存放闭包并在 value 中存放 Option 值
struct Cacher<T> 
where
	T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
```

`Cacher`的缓存逻辑：

```rust
impl<T> Cacher<T>
where
	T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

`Cacher` 结构体的字段是私有的，因为我们希望 `Cacher` 管理这些值而不是任由调用代码潜在的直接改变他们。

`Cacher::new` 函数获取一个泛型参数 `T`，它定义于 `impl` 块上下文中并与 `Cacher` 结构体有着相同的 trait bound。`Cacher::new` 返回一个在 `calculation` 字段中存放了指定闭包和在 `value` 字段中存放了 `None` 值的 `Cacher` 实例，因为我们还未执行闭包。

当调用代码需要闭包的执行结果时，不同于直接调用闭包，它会调用 `value` 方法。这个方法会检查 `self.value` 是否已经有了一个 `Some` 的结果值；如果有，它返回 `Some` 中的值并不会再次执行闭包。

如果 `self.value` 是 `None`，则会调用 `self.calculation` 中储存的闭包，将结果保存到 `self.value` 以便将来使用，并同时返回结果值。

不同于直接将闭包保存进一个变量，我们保存一个新的 `Cacher` 实例来存放闭包。接着，在每一个需要结果的地方，调用 `Cacher` 实例的 `value` 方法。可以调用 `value` 方法任意多次，或者一次也不调用，而慢计算最多只会运行一次。

#### Cacher实现的限制

```rust
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
// 结果会报错
// 这里的问题是第一次使用 1 调用 c.value，Cacher 实例将 Some(1) 保存进 self.value。在这之后，无论传递什么值调用 value，它总是会返回 1。
```

#### 闭包会捕获其环境

我们可以将闭包作为内联匿名函数来使用。闭包还有另一个函数所没有的功能：他们可以捕获其环境并访问其被定义的作用域的变量。

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

这里，即便 `x` 并不是 `equal_to_x` 的一个参数，`equal_to_x` 闭包也被允许使用变量 `x`，因为它与 `equal_to_x` 定义于相同的作用域。  函数则不行。例如：

```rust
fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool {
        z == x
    }

    let y = 4;

    assert!(equal_to_x(y));
}
// 会报错
```

闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。这三种捕获值的方式被编码为如下三个 `Fn` trait：

- `FnOnce` 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 **环境**，*environment*。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 `Once` 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
- `FnMut` 获取可变的借用值所以可以改变其环境
- `Fn` 从其环境获取不可变的借用值

即使其捕获的值已经被移动了，`move` 闭包仍需要实现 `Fn` 或 `FnMut`。这是因为闭包所实现的 trait 是由闭包所捕获了什么值而不是如何捕获所决定的。而 `move` 关键字仅代表了后者。

```rust
let equal_to_x = move |z| z == x;  
// 会获取x的所有权
```

### 使用迭代器处理元素序列

迭代器模式允许你对一个序列的项进行某些处理。**迭代器**（*iterator*）负责遍历序列中的每一项和决定序列何时结束的逻辑。

在Rust中，迭代器是惰性的(lazy)，这意味着在调用方法使用迭代器之前它都不会有效果。

```rust
// 在一个for循环中使用迭代器
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
for val in v1_iter {
    println!("Got: {}", val);
}
```

迭代器的实现方式提供了对多种不同的序列使用相同逻辑的灵活性，而不仅仅是像 vector 这样可索引的数据结构。

#### Iterator trait 和 next 方法

迭代器都实现了一个叫做 Iterator 的定义于标准库的trait。

```rust
pub trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
    // 省略方法的默认实现
}
```

`type Item`和`Self::Item`，他们定义了 trait 的关联类型(associated type)

这段代码表明实现 `Iterator` trait 要求同时定义一个 `Item` 类型，这个 `Item` 类型被用作 `next` 方法的返回值类型。换句话说，`Item` 类型将是迭代器返回元素的类型。

注意 `v1_iter` 需要是可变的：在迭代器上调用 `next` 方法改变了迭代器中用来记录序列位置的状态。换句话说，代码 **消费**（consume）了，或使用了迭代器。每一个 `next` 调用都会从迭代器中消费一个项。

#### 消费迭代器的方法

这些调用 `next` 方法的方法被称为 **消费适配器**（*consuming adaptors*），因为调用他们会消耗迭代器。一个消费适配器的例子是 `sum` 方法。这个方法获取迭代器的所有权并反复调用 `next` 来遍历迭代器，因而会消费迭代器。当其遍历每一个项时，它将每一个项加总到一个总和并在迭代完成时返回总和。

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}
```

调用 `sum` 之后不再允许使用 `v1_iter` 因为调用 `sum` 时它会获取迭代器的所有权。

#### 产生其他迭代器的方法

`Iterator` trait 中定义了另一类方法，被称为 **迭代器适配器**（*iterator adaptors*），他们允许我们将当前迭代器变为不同类型的迭代器。可以链式调用多个迭代器适配器。不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。

```rust
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
```

#### 使用闭包获取环境

迭代器的 `filter` 方法获取一个使用迭代器的每一个项并返回布尔值的闭包。如果闭包返回 `true`，其值将会包含在 `filter` 提供的新迭代器中。如果闭包返回 `false`，其值不会包含在结果迭代器中。

#### 实现 Iterator trait 来创建自定义迭代器

所有这些方法调用都是可能的，因为我们指定了 `next` 方法如何工作，而标准库则提供了其它调用 `next` 的方法的默认实现。

迭代器，作为一个高级的抽象，被编译成了与手写的底层代码大体一致性能代码。迭代器是 Rust 的 **零成本抽象**（*zero-cost abstractions*）之一，它意味着抽象并不会引入运行时开销



## 智能指针

**智能指针**（*smart pointers*）是一类数据结构，他们的表现类似指针，但是也拥有额外的元数据和功能。

在 Rust 中，普通引用和智能指针的一个额外的区别是引用是一类只借用数据的指针；相反，在大部分情况下，智能指针拥有他们指向的数据。

来自标准库中最常用的一些智能指针：

- `Box<T>`，用于在堆上分配值
- `Rc<T>`，一个引用计数类型，其数据可以有多个所有者
- `Ref<T>` 和 `RefMut<T>`，通过 `RefCell<T>` 访问。（ `RefCell<T>` 是一个在运行时而不是在编译时执行借用规则的类型）。

另外我们会涉及 **内部可变性**（*interior mutability*）模式，这是不可变类型暴露出改变其内部值的 API。也会讨论 **引用循环**（*reference cycles*）会如何泄漏内存，以及如何避免。

### 使用`Box<T>`指向堆上数据

智能指针box，其类型是`Box<T>。box 允许将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。

#### 使用 `Box<T>` 在堆上储存数据

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

#### Box 允许创建递归类型

Rust 需要在编译时知道类型占用多少空间。一种无法在编译时知道大小的类型是 **递归类型**（*recursive type*），box 有一个已知的大小，所以通过在循环类型定义中插入 box，就可以创建递归类型了。

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

`Cons` 成员将会需要一个 `i32` 的大小加上储存 box 指针数据的空间。`Nil` 成员不储存值，所以它比 `Cons` 成员需要更少的空间。现在我们知道了任何 `List` 值最多需要一个 `i32` 加上 box 指针数据的大小。

![A finite Cons list](https://kaisery.github.io/trpl-zh-cn/img/trpl15-02.svg)

box 只提供了间接存储和堆分配；他们并没有任何其他特殊的功能。

`Box<T>` 类型是一个智能指针，因为它实现了 `Deref` trait，它允许 `Box<T>` 值被当作引用对待。当 `Box<T>` 值离开作用域时，由于 `Box<T>` 类型 `Drop` trait 的实现，box 所指向的堆数据也会被清除。

### 使用 Deref Trait 将智能指针当作常规引用处理

```rust
fn main() {
    let x = 5;
    let y = &x;
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

像引用一样使用 `Box<T>`

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

#### 自定义智能指针

```rust
// 定义 MyBox(x) 类型和 new() 方法
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

`MyBox<T>` 类型不能解引用，因为我们尚未在该类型实现这个功能。为了启用 `*` 运算符的解引用功能，需要实现 `Deref` trait。

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

`type Target = T;` 语法定义了用于此 trait 的关联类型。关联类型是一个稍有不同的定义泛型参数的方式

`deref` 方法体中写入了 `&self.0`，这样 `deref` 返回了我希望通过 `*` 运算符访问的值的引用。`0` 用来访问元组结构体的第一个元素。

输入 `*y` 时，实际运行了 `*(y.deref())`，Rust 将 * 运算符换为先调用 deref 方法再进行普通解引用的操作，就不用担心是否还需手动调用 deref 方法了。

#### 函数和方法的隐式 Deref 强制转换

Deref 强制转换 (deref coercions) 是Rust在函数或方法传参上的一种便利。Deref 强制转换只能作用于实现了 Deref trait 的类型。Deref 强制转换将这样一个类型的引用转换为另一个类型的引用。

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    // Deref 强制转换，使用 MyBox<String> 的引用调用 hello 是可行的
    hello(&m);
    // hello(&(*m)[..]);
    // (*m) 将 MyBox<String> 解引用为 String。
    // 接着 & 和 [..] 获取了整个 String 的字符串 slice 来匹配hello的签名。
}
```

当所涉及到的类型定义了 `Deref` trait，Rust 会分析这些类型并使用任意多次 `Deref::deref` 调用以获得匹配参数的类型。这些解析都发生在编译时，所以利用 Deref 强制转换并没有运行时损耗！

Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：

- 当 `T: Deref<Target=U>` 时从 `&T` 到 `&U`。
- 当 `T: DerefMut<Target=U>` 时从 `&mut T` 到 `&mut U`。
- 当 `T: Deref<Target=U>` 时从 `&mut T` 到 `&U`。

### 使用 Drop Trait 运行清理代码

`drop` 函数体是放置任何当类型实例离开作用域时期望运行的逻辑的地方。

```rust
// 结构体 CustomSmartPointer，其实现了放置清理代码的 Drop trait
struct CustomSmartPointer {
    data: String,
}

// drop 函数体是放置任何当类型实例离开作用域时期望运行的逻辑的地方。
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// Drop trait 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。
// 为了能够看出 Rust 何时调用 drop，让我们暂时使用 println! 语句实现 drop。
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

// 结果是   后drop 先丢弃
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

注意无需显式调用 `drop` 方法。

当实例离开作用域 Rust 会自动调用 `drop`，并调用我们指定的代码。变量以被创建时相反的顺序被丢弃，所以 `d` 在 `c` 之前被丢弃。这个例子刚好给了我们一个 drop 方法如何工作的可视化指导，不过通常需要指定类型所需执行的清理代码而不是打印信息。

#### 通过 `std::mem::drop` 提早丢弃值

Rust 并不允许我们主动调用 `Drop` trait 的 `drop` 方法；当我们希望在作用域结束之前就强制释放变量的话，我们应该使用的是由标准库提供的 `std::mem::drop`。

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

表明了 `drop` 方法被调用了并在此丢弃了 `c`。

### `Rc<T>`引用计数智能指针

为了启用多所有权，Rust 有一个叫做 `Rc<T>` 的类型。其名称为 **引用计数**（*reference counting*）的缩写。引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。

注意 `Rc<T>` 只能用于单线程场景。

![Two lists that share ownership of a third list](https://kaisery.github.io/trpl-zh-cn/img/trpl15-03.svg)

用 Rc 代替 Box ，因为 Box 会移交列表所有权，导致报错。

当创建 `b` 时，不同于获取 `a` 的所有权，这里会克隆 `a` 所包含的 `Rc<List>`，这会将引用计数从 1 增加到 2 并允许 `a` 和 `b` 共享 `Rc<List>` 中数据的所有权。创建 `c` 时也会克隆 `a`，这会将引用计数从 2 增加为 3。每次调用 `Rc::clone`，`Rc<List>` 中数据的引用计数都会增加，直到有零个引用之前其数据都不会被清理。

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

`Rc::clone` 的实现并不像大部分类型的 `clone` 实现那样对所有数据进行深拷贝。`Rc::clone` 只会增加引用计数，这并不会花费多少时间。深拷贝可能会花费很长时间。通过使用 `Rc::clone` 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。

#### 克隆`Rc<T>`会增加引用计数

 `a` 中 `Rc<List>` 的初始引用计数为1，接着每次调用 `clone`，计数会增加1。当 `c` 离开作用域时，计数减1。不必像调用 `Rc::clone` 增加引用计数那样调用一个函数来减少计数；`Drop` trait 的实现当 `Rc<T>` 值离开作用域时自动减少引用计数。

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

通过不可变引用， `Rc<T>` 允许在程序的多个部分之间只读地共享数据。

### `RefCell<T>`与内部可变性模式

内部可变性(Interior mutability) 是 Rust 中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的。

#### 通过`RefCell<T>`在运行时检查借用规则

不同于`Rc<T>`，`RefCell<T>`代表其数据的唯一的所有权。

借用规则：

1. 在任意给定时刻，只能拥有一个可变引用或任意数量的不可变引用之一（而不是两者）。
2. 引用必须总是有效的。

如下为选择 `Box<T>`，`Rc<T>` 或 `RefCell<T>` 的理由：

- `Rc<T>` 允许相同数据有多个所有者；`Box<T>` 和 `RefCell<T>` 有单一所有者。
- `Box<T>` 允许在编译时执行不可变或可变借用检查；`Rc<T>`仅允许在编译时执行不可变借用检查；`RefCell<T>` 允许在运行时执行不可变或可变借用检查。
- 因为 `RefCell<T>` 允许在运行时执行可变借用检查，所以我们可以在即便 `RefCell<T>` 自身是不可变的情况下修改其内部的值。

在不可变值内部改变值就是 **内部可变性** 模式。

### 引用循环会导致内存泄漏

使用智能指针来做出不同于 Rust 常规引用默认所提供的保证与取舍。`Box<T>` 有一个已知的大小并指向分配在堆上的数据。`Rc<T>` 记录了堆上数据的引用数量以便可以拥有多个所有者。`RefCell<T>` 和其内部可变性提供了一个可以用于当需要不可变类型但是需要改变其内部值能力的类型，并在运行时而不是编译时检查借用规则。



## 无畏并发

### 使用线程同时地运行代码

在大部分现代操作系统中，已执行程序地代码在一个进程(process)中运行，操作系统则会负责管理多个进程。在程序内部，也可以拥有多个同时运行地独立部。这些运行这些独立部分地功能被称为线程(threads)。

#### 使用`spawn`创建新线程

为了创建一个新线程，需要调用`thread::spawn`函数并传递一个闭包，并在其中包含希望在新线程运行的代码。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

#### 使用`join`等待所有线程结束

由于上面`spawn`无法保证新建线程执行完，可以通过将`thread::spawn`的返回值储存在变量中来修复新建线程部分没有执行或者完全没有执行的问题。

`thread::spawn`的返回值类型是`JoinHandle`。`JoinHandle`是一个拥有所有权的值，当对其调用`join`方法时，它会等待其线程结束。如下所示：

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

#### 线程与move闭包

`move`关键字经常用于传递给`thread::spawn`的闭包，因为闭包会获取从环境中取得的值的所有权，因此会将这些值的所有权从一个线程传送到另一个线程。

可以在参数列表前使用`move`关键字强制闭包获取其使用的环境值的所有权。这个技巧在创建新县城将值的所有权从一个线程移动到另一个线程是最为实用。

使用`move`关键字强制获取它使用的值的所有权

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    
    handle.join().unwrap();
}
```

### 使用消息传递在线程间通信

为了实现消息传递并发，Rust标准库提供了一个信道(channel)实现，表示数据从一个线程发送到另一个线程。

编程中的信道由两部分组成，一个发送者(transmitter)和一个接收者(receiver)。当发送者或接收者任一被丢弃时可以认为信道被关闭(closed)了。

使用 `mpsc::channel` 函数创建一个新的信道；`mpsc` 是 **多个生产者，单个消费者**（*multiple producer, single consumer*）的缩写。简而言之，Rust 标准库实现信道的方式意味着一个信道可以有多个产生值的 **发送**（*sending*）端，但只能有一个消费这些值的 **接收**（*receiving*）端。

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

信道的接收者有两个有用的方法：`recv` 和 `try_recv`。这里，我们使用了 `recv`，它是 *receive* 的缩写。这个方法会阻塞主线程执行直到从信道中接收一个值。一旦发送了一个值，`recv` 会在一个 `Result<T, E>` 中返回它。当信道发送端关闭，`recv` 会返回一个错误表明不会再有新的值到来了。

`try_recv` 不会阻塞，相反它立刻返回一个 `Result<T, E>`：`Ok` 值包含可用的信息，而 `Err` 值代表此时没有任何消息。如果线程在等待消息过程中还有其他工作时使用 `try_recv` 很有用：可以编写一个循环来频繁调用 `try_recv`，在有可用消息时进行处理，其余时候则处理一会其他工作直到再次检查。

#### 信道与所有权转移

`send` 函数获取其参数的所有权并移动这个值归接收者所有。

在 `tx.send(val).unwrap();`之后就无法再使用了，例如 `println!("val is {}", val);`会有问题。

#### 发送多个值并观察接收者的等待

新建线程现在会发送多个消息并在每个消息之间暂停一秒钟。

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

在主线程中，不再显式调用 `recv` 函数：而是将 `rx` 当作一个迭代器。对于每一个接收到的值，我们将其打印出来。当信道被关闭时，迭代器也将结束。

可以说主线程是在等待从新建线程中接收值。

#### 通过克隆发送者来创建多个生产者

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

### 共享状态并发

Rust中，得益于类型系统和所有权，使得通过共享内存实现并发安全实现。

共享内存类似于多所有权：多个线程可以同时访问相同的内存位置。

#### 互斥器一次只允许一个线程访问数据

互斥器(mutex) 是*mutual exclusion*的缩写，即任意时刻，其只允许一个线程访问某些数据。锁(lock)是一个作为互斥器一部分的数据结构，它记录谁有数据的排他访问权。因此，我们描述互斥器为通过锁系统保护(guarding)其数据。

在其他语言中，互斥器难以使用在于：

1. 在使用数据之前尝试获取锁。
2. 处理完被互斥器所保护的数据之后，必须解锁数据，这样其他线程才能够获取锁。

#### 多线程和多所有权

原子引用计数 `Arc`

`Arc<T>` **正是** 这么一个类似 `Rc<T>` 并可以安全的用于并发环境的类型。字母 “a” 代表 **原子性**（*atomic*），所以这是一个 **原子引用计数**（*atomically reference counted*）类型。

性能安全带有性能惩罚，我们只希望在必要时为其买单。如果只是在单线程中对值进行操作，原子性提供的保证并无必要，代码可以因此运行的更快。

```rust
// 多线程和多所有权
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    // 收集了所有的 join 句柄，调用它们的 join 方法来确保所有线程都会结束。
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

### 使用`Sync`和`Send trait`的可扩展并发

有两个并发概念是内嵌于语言中的：`std::marker` 中的 `Sync` 和 `Send` trait。

#### 通过`Send`允许在线程间转移所有权

`Send` 标记 trait 表明实现了 `Send` 的类型值的所有权可以在线程间传送。

任何完全由`Send`的类型组成的类型也会自动标记为`Send`。几乎所有的基本类型都是`Send`的，除了裸指针(raw pointer)。

#### `Sync`允许多线程访问

`Sync`标记trait表明一个实现了`Sync`的类型可以安全的在多个线程中拥有其值的引用。

对于任意类型T，如果&T（T的不可变引用）是`Send`的话T就是`Sync`的，这意味着其引用就可以安全的发送到另一个线程。例如 `Mutex<T>`是`Sync`的，可以被用来在多线程中共享访问。

#### 手动实现`Send`和`Sync`是不安全的

通常不需要手动实现`Send`和`Sync` trait，因为由`Send`和`Sync`的类型组成的类型，自动就是`Send`和`Sync`的。因为他们是标记trait，甚至都不需要实现任何方法。他们只是用来加强并发相关的不可变性的。

手动实现这些trait涉及到编写不安全的Rust代码。

Rust 提供了用于消息传递的信道，和像 `Mutex<T>` 和 `Arc<T>` 这样可以安全的用于并发上下文的智能指针。类型系统和借用检查器会确保这些场景中的代码，不会出现数据竞争和无效的引用。一旦代码可以编译了，我们就可以坚信这些代码可以正确的运行于多线程环境。



## Rust 的面向对象编程特点

### 面向对象语言的特点

#### 对象包含数据和行为

面向对象的程序是由对象组成的。一个对象包含数据和操作这些数据的过程。这些过程通常被称为方法或操作。

在这个定义下，Rust是面向对象的：结构体和枚举包含数据而impl块提供了在结构体和枚举之上的方法。虽然带有方法的结构体和枚举并不被称为对象，但是他们提供了与对象相同的功能。

#### 封装隐藏了实现细节

结构体自身被标记为 `pub`，这样其他代码就可以使用这个结构体，但是在结构体内部的字段仍然是私有的。这是非常重要的，因为我们希望保证变量被增加到列表或者被从列表删除时，也会同时更新平均值。可以通过在结构体上实现 `add`、`remove` 和 `average` 方法来做到这一点。

#### 继承，作为类型系统与代码共享

继承(Inheritance)是一个很多编程语言都提供的机制，一个对象可以定义为继承另一个对象定义中的元素，这使其可以获得父对象的数据和行为，而无需重新定义。

选择继承有两个主要的原因。第一个是为了重用代码：一旦为一个类型实现了特定行为，继承可以对一个不同的类型重用这个实现。Rust 代码中可以使用默认 trait 方法实现来进行有限的共享。

第二个使用继承的原因与类型系统有关：表现为子类型可以用于父类型被使用的地方。这也被称为 **多态**（*polymorphism*），这意味着如果多种对象共享特定的属性，则可以相互替代使用。

### 为使用不同类型的值而设计的`trait`对象

#### 定义通用行为的trait

我们可以使用 trait 对象代替泛型或具体类型。任何使用 trait 对象的位置，Rust 的类型系统会在编译时确保任何在此上下文中使用的值会实现其 trait 对象的 trait。如此便无需在编译时就知晓所有可能的类型。

在结构体或枚举中，结构体字段中的数据和 `impl` 块中的行为是分开的，不同于其他语言中将数据和行为组合进一个称为对象的概念中。trait 对象将数据和行为两者相结合，从这种意义上说 **则** 其更类似其他语言中的对象。不过 trait 对象不同于传统的对象，因为不能向 trait 对象增加数据。trait 对象并不像其他语言中的对象那么通用：其（trait 对象）具体的作用是允许对通用行为进行抽象。

#### trait 对象需要类型安全

只有对象安全(object-safe)的trait可以实现为特征对象。如果一个trait中定义的所有方法都符合一下规则，则该trait是对象安全的：

- 返回值不是`Self`
- 没有泛型类型的参数

`Self`关键字是我们在trait与方法上的实现的别称，trait对象必须是对象安全的，因为一旦使用trait对象，Rust将不再知晓该实现的返回类型。如果一个trait的方法返回了一个`Self`类型，但是该trait对象忘记了`Self`的确切类型，那么该方法将不能使用原本的类型。

### 面向对象设计模式的实现

**状态模式**(state pattern)是一个面向对象设计模式。该模式的关键在于定义一系列的内含状态。这些状态体现为一系列的**状态对象**，同时值的行为随着其内部状态而改变。

状态对象共享功能：当然，在Rust中使用结构体和trait而不是对象和继承。每一个状态对象负责其自身的行为，以及该状态何时应当转移至另一个状态。持有一个状态对象的值对于不同状态的行为以及何时状态转移毫不知情。

```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```

#### 定义Post并新建一个草案状态的实例

```rust
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
```

#### 存放博文内容的文本

实现方法`add_text`来向博文的`content`增加文本

```rust
impl Post {
	// --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

#### 确保博文草案的内容是空的

增加一个Post的content方法的占位实现，总是返回一个空字符串slice

```rust
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        ""
    }
}
```

#### 请求审核博文来改变其状态

增加请求审核博文的功能，这应当将其状态由`Draft`改为`PendingReview`。

```rust
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // --snip--
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        ""
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

为了消费老状态，`request_review` 方法需要获取状态值的所有权。这就是 `Post` 的 `state` 字段中 `Option` 的来历：调用 `take` 方法将 `state` 字段中的 `Some` 值取出并留下一个 `None`，因为 Rust 不允许结构体实例中存在值为空的字段。这使得我们将 `state` 的值移出 `Post` 而不是借用它。接着我们将博文的 `state` 值设置为这个操作的结果。

#### 增加改变content行为的approve方法



## 模式与模式匹配

模式（Patterns）是Rust中特殊的语法，它用来匹配类型中的结构，无论简单类型还是复杂类型。

模式由如下一些内容组合而成：

- 字面值
- 解构的数组、枚举、结构体或者元组
- 变量
- 通配符
- 占位符

### 所有可能会用到模式的位置

#### `match`分支

一个模式常用的位置是`match`表达式的分支。在形式上`match`表达式由`match`关键字、用于匹配的值和一个或多个分支构成，这些分支包含一个模式和在值匹配分支的模式时运行的表达式：

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

例如：

```rust
// 这个match表达式的模式为每个箭头左边的None和Some(i)
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

`match`表达式必须是**穷尽**(exhaustive)的，`match`表达式所有可能的值都必须被考虑到。

一个确保覆盖每个可能值的方法是在最后一个分支使用捕获所有的模式：比如，一个匹配任何值的名称永远也不会失败，因此可以覆盖所有匹配剩下的情况。

有一个特定的模式 `_` 可以匹配所有情况，不过它从不绑定任何变量。

#### if let 条件表达式

```rust
fn main() {
    let favorite_color: Option<&str> None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the backgroud");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

#### `while let`条件循环

`while let`条件循环，允许只要模式匹配就一直进行`while`循环。

```rust
// 使用 while let 循环只要 stack.pop() 返回Some就打印出其值
// 使用 while let 来弹出栈中的每一个元素
fn main() {
    let mut stack = Vec::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

#### for 循环

在`for`循环中，模式是`for`关键字直接跟随的值，正如`for x in y`中的`x`。

```rust
// 使用 for 循环来解构，或拆开一个元组作为 for 循环的一部分
fn main() {
    let v = vec!['a', 'b', 'c'];
    
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
```

这里使用的`enumerate`方法适配一个迭代器来产生一个值和其在迭代器中的索引，他们位于一个车元组中。第一个产生的值是元组`(0, 'a')`。当这个值匹配模式(index, value)，打印。

#### let 语句

`let`表达式也是使用模式：

```rust
let x = 5;
let PATTERN = EXPRESSION;
```

 将表达式与模式比较，并为任何找到的名称赋值。这个模式实际上等于 “将任何值绑定到变量x，不管值是什么”。

```rust
let (x, y, z) = (2, 3, 4);
```

#### 函数参数

函数参数也可以是模式。

```rust
// x 部分就是一个模式
fn foo(x: i32) {
    
}

// 一个在参数中解构元组的函数
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

在一些地方，模式必须是irrefutable的，意味着他们必须匹配所提供的任何值。在另一些情况，他们则可以是refutable的。

### Refutability（可反驳性）：模式是否会匹配失效

模式又两种形式：refutable（可反驳的）和irrefutable（不可反驳的）。

能匹配任何传递的可能值的模式被称为是**不可反驳的**（irrefutable）。例如`let x = 5`；因为x可以匹配任何值所以不可能会失败。

对某些可能的值进行匹配会失败的模式被称为是可反驳的（refutable）例如`if let Some(x) = a_value`表达式中的`Some(x)`，如果变量`a_value`中的值是`None`而不是`Some`，那么`Some(x)`模式不能匹配。

`let`语句只能接受不可反驳模式。为了修复在需要不可反驳模式的地方使用可反驳模式的情况，可以修改使用模式的代码：不同于使用`let`，可以使用`if let`。如此，如果模式不匹配，大括号中的代码将被忽略，其余代码保持有效。

```rust
if let Some(x) = some_option_value {
    println!("{}", x);
}
```

`match`匹配分支必须使用可反驳模式，除了最后一个分支需要使用能匹配任何剩余值的不可反驳模式。

### 模式语法

#### 匹配字面值

可以直接匹配字面值模式

```rust
fn main() {
    let x = 1;
    
    match x {
        1 => println!("one");
        2 => println!("two");
        _ => println!("anything");
    }
}
```

#### 匹配命名变量

命名变量是匹配任何值的不可反驳模式。当其用于`match`表达式时情况会有些复杂，因为`match`会开始一个新作用域，`match`表达式中作为模式的一部分生命的变量会覆盖`match`结构之外的同名变量。

```rust
fn main() {
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        - => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);
}
```

























































