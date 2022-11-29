# Rust function

## 1、Rust 表达式

函数调用是一个表达式，宏调用是一个表达式。

用大括号创建的一个新的块作用域也是一个表达式，例如：

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of y is: {y}");
}
```

其中表达式：

```rust
{
    let x = 3;
    x + 1
}
```

是一个代码块， 值为4。这个值作为`let`语句的一部分被绑定到`y`上。

表达式`x + 1`这一行的结尾没有分号。在表达式的结尾加上分号，就变成了语句，而语句不会返回值。



## 2、具有返回值的函数

函数可以向调用它的代码返回值。没有对返回值进行命名，但要在箭头(->)后声明它的类型。

在Rust中，函数的返回值等同于函数体最后一个表达式的值。使用`return`关键字和指定值，可从函数中提前返回；大部分函数隐式的返回最后的表达式。



## 3、循环标签：在多个循环之间消除歧义

如果存在嵌套循环，`break` 和 `continue` 应用于此时最内层的循环。你可以选择在一个循环上指定一个 **循环标签**（*loop label*），然后将标签与 `break` 或 `continue` 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环。

```rust
fn for_exp() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```



### 方法 rev  用来反转range

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
```

