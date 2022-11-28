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

### 2. 复合类型(Compound types)

> 可以将多个值组合成一个类型。 Rust 有两个原生的复合类型： 元组(tuple) 和数组(array)。

- **元组类型**

  元组是一个将多个其他类型的值组合成一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。

  我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的。

  ```rust
  fn main() {
      let tup: (i32, f64, u8) = (500, 6.4, 1);
  }
  ```

  `tup` 变量绑定到整个元组上，因为元组是一个单独的复合元素。为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值

  ```rust
  fn main() {
      let tup = (500, 6.4, 1);
      
      let (x, y, z) = tup;
      
      println!("The value of y is: {y}");
  }
  ```

  我们也可以使用点号（`.`）后跟值的索引来直接访问它们。

  ```rust
  fn main() {
      let x: (i32, f64, u8) = (500, 6.4, 1);
      
      let five_hundred = x.0;
      let six_point_four = x.1;
      let one = x.2;
  }
  ```

  不带任何值的元组有个特殊的名称，叫做 **单元（unit）** 元组。这种值以及对应的类型都写作 `()`，表示空值或空的返回类型。如果表达式不返回任何其他值，则会隐式返回单元值。



- **数组类型**

  Rust中数组的长度是固定的，每个元素的类型必须相同。将数组的值写成在方括号内， 用逗号分隔：

  ```rust
  fn main() {
      let a = [1, 2, 3, 4, 5];
      
      let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];
  }
  
  ```

  数组在栈上而不是在堆上为数据分配空间，确保总是有固定数量的元素。

  可以像这样编写数组的类型：在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。

  ```rust
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  
  // 或者方括号中进行处理, 指定初始值加分号再加元素个数的方式来创建一个每个元素都为相同值的数组：
  let a = [3; 5];
  ```

  **数组是可以在栈(stack)上分配的已知固定大小的单个内存块。可以使用索引来访问数组的元素**

  ```rust
  fn main() {
      let a = [1, 2, 3, 4, 5];
      let first = a[0];
      let second = a[1];
  }
  ```

  

vector 类型是标准库提供的一个 **允许** 增长和缩小长度的类似数组的集合类型。当不确定是应该使用数组还是 vector 的时候，那么很可能应该使用 vector。