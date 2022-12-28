use std::fs;
use std::error::Error;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 存储匹配的行
    let mut results = Vec::new();

    for line in contents.lines() {
        // 对文本进行操作
        if line.contains(query) {
            // 对文本行进行操作
            results.push(line);
        }
    }

    results
}


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

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    // println!("With text:\n{}", contents);

    Ok(())
}