use std::fs;
use std::error::Error;

// !!!!!注意pub!!!!!

// 组合配置值
// 在之前的 parse_config 函数体中，
// 我们返回了引用 args 中 String 值的字符串 slice，
// 现在我们定义 Config 来包含拥有所有权的 String 值。
pub struct Config {
    pub query: String,
    pub filename: String,
}

// 创建一个 Config 的构造函数
// (从 new 中返回 Result 而不是调用 panic!)
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        // 改善错误信息
        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}


// Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，
// 不过无需指定具体将会返回的值的类型。
// 这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。
// 这也就是 dyn，它是 “动态的”（“dynamic”）的缩写。

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //let contents = fs::read_to_string(config.filename)
    //    .expect("Something went wrong reading the file");

    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}