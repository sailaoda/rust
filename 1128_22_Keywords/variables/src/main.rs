fn main() {
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // mut 变量可变
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");

    let x = x + 1;

    {
        // 另一个作用域
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
