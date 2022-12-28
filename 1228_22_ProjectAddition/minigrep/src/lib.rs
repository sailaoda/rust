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
Pick three.
Duck tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // 为准备添加的大小写不敏感函数新增失败测试
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
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

// 会将query变量和每一line都变为小写。
// 不管输入参数大写小写，在检查该行是否包含查询字符串时都会是小写。
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
    pub case_sensitive: bool,
}

use std::env;

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

        // Ok(Config { query, filename })

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
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

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    /* for line in search(&config.query, &contents) {
        println!("{}", line);
    } */
    for line in results {
        println!("{}", line);
    }
    // println!("With text:\n{}", contents);

    Ok(())
}