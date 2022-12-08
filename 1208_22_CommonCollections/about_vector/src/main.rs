
fn main() {

    // 创建 vector
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    // 更新 vector
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);

    // 丢弃vector时也会丢弃其所有元素
    {
        let v4 = vec![1, 2, 3, 4];
        // 处理变量 v4
    }   // 这里 v 离开作用域并被丢弃

    // 读取vector的元素：索引语法或者get语法
    {
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2];
        println!("The third element is {}", third);
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }

    {
        let v = vec![1, 2, 3, 4, 5];
        // 当[]不存在的索引时， 会panic
        // let does_not_exist = &v[100];
        // get() 不存在的索引时，不会panic，会返回 None
        let does_not_exist = v.get(100);
    }

    // 在拥有 vector中项的引用的同时向其增加一个元素
    /* {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        v.push(6);
        println!("The first element is: {}", first);
    } */

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

    // 使用枚举来存放多种类型
    {
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        for i in &row {
            println!("{:?}", i);
        }
    }

    println!("Hello, world!");
}
