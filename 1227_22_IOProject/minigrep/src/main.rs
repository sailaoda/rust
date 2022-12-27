use std::env;
use std::fs;

/* 注意 std::env::args 在其任何参数包含无效 Unicode 字符时会 panic。
如果你需要接受包含无效 Unicode 字符的参数，使用 std::env::args_os 代替。
这个函数返回 OsString 值而不是 String 值。
这里出于简单考虑使用了 std::env::args，
因为 OsString 值每个平台都不一样而且比 String 值处理起来更为复杂。 */
fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let filename = &args[2];
    // let (query, filename) = parse_config(&args);
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);



    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// 组合配置值
// 在之前的 parse_config 函数体中，
// 我们返回了引用 args 中 String 值的字符串 slice，
// 现在我们定义 Config 来包含拥有所有权的 String 值。
struct Config {
    query: String,
    filename: String,
}

// 提取参数解析器
/* fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
} */
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {query, filename}
}
