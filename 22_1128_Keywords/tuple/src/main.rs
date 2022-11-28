fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup =(500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{z}");
    let the_first = tup.1;
    println!("the first tuple is {the_first}");
}
