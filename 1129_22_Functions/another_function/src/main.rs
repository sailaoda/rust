fn main() {
    println!("Hello, world!");

    another_function();

    parameter(5);

    labeled(5, 'h');
}

fn another_function() {
    println!("Another function.");
}

fn parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn labeled(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Rust 不能连续赋值   如 x = y = 5 不能这样写