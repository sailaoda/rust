fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn main() {
    let number_list = vec![34, 54, 23, 655, 23];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let mut number_list2 = vec![23, 45, 454, 341, 322, 555];

    number_list2.push(result);
    let result2 = largest(&number_list2);
    println!("The largest number is {}", result2)
}
