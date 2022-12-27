use std::env;
use std::fs;
use std::process;

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

    // let config = parse_config(&args);
    // let config = Config::new(&args);


    // Config::new 调用并处理错误
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        // 手动实现原先由 panic!负责的工作，即以非零错误码退出命令行工具的工作。
        // 非零的退出状态是一个惯例信号，
        // 用来告诉调用程序的进程：该程序以错误状态退出了。
        process::exit(1);
    });

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

// 创建一个 Config 的构造函数
// (从 new 中返回 Result 而不是调用 panic!)
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {

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
