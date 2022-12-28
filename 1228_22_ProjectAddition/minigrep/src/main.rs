use std::env;

use std::process;

use minigrep::Config;

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
        // println!("Problem parsing arguments: {}", err);
        eprintln!("Problem parsing arguments: {}", err);
        // 手动实现原先由 panic!负责的工作，即以非零错误码退出命令行工具的工作。
        // 非零的退出状态是一个惯例信号，
        // 用来告诉调用程序的进程：该程序以错误状态退出了。
        process::exit(1);
    });

    /* println!("Searching for {}", config.query);
    println!("In file {}", config.filename); */


    // let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    // println!("With text:\n{}", contents);
    // 从main提取逻辑
    // run(config);

    if let Err(e) = minigrep::run(config) {
        // println!("Application error: {}", e);
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

// 从main提取逻辑
// 提取 run 函数来包含剩余的程序逻辑
// 从run中返回错误





// 提取参数解析器
/* fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
} */
/* fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {query, filename}
} */


