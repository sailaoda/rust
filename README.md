

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





