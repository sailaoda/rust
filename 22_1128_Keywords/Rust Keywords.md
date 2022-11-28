# Rust Keywords

## 一、变量和可变性

> `mut` 标识使变量可变， let mut x = 2;

- 常量(constants) 

  ```
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  ```

  常量的命名，大写字母加下划线

- 隐藏(shadowing)

  变量可以使用`let`关键字多次隐藏， let的隐藏和mut的可变是有区别的

  ```rust
  fn main() {
      // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
      // mut 变量可变
      let mut x = 5;
      println!("The value of x is : {x}");
      x = 6;
      println!("The value of x is : {x}");
  
      // 隐藏需要let， 且使用let实际上创建了一个新变量， 不过新变量是不可变的
      let x = x + 1;
      {
          // 另一个作用域
          let x = x * 2;
          println!("The value of x in the inner scope is: {x}");
      }
      println!("The value of x is: {x}");
  }
  ```

  不能改变变量的类型



## 二、 数据类型(data type)

> 有两类数据类型子集： 标量(scalar)  和 复合(compound)
>
> Rust 是静态类型(statically typed)语言，即在编译时就必须知道所有变量的类型。

使用`parse`将`String`转换为数字是， 必须增加类型注解：

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

### 1. 标量类型(scalar)

> Rust 有四种标量类型：整型、浮点型、布尔类型和字符类型







