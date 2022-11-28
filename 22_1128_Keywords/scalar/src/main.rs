fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 20;
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
                         // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("these result are sum: {sum},\n difference: {difference},\n product: {product},\n quotient: {quotient},\n");

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");
}
