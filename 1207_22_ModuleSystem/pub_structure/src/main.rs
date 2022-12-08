
// 带有公有和私有字段的结构体

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// 枚举
mod back_of_house1 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {

    let order1 = back_of_house1::Appetizer::Soup;
    let order2 = back_of_house1::Appetizer::Salad;

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I`d like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");
}

// 给枚举的所有成员挨个添加 pub 是很令人恼火的，因此枚举成员默认就是公有的。
// 结构体通常使用时，不必将它们的字段公有化，因此结构体遵循常规，内容全部是私有的，除非使用 pub 关键字。

fn main() {}