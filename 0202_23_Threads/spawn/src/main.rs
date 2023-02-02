use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 当 Rust 程序的主线程结束时，新线程也会结束，而不管其是否执行完毕。
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        // thread::sleep 调用强制线程停止执行一小段时间，这会允许其他不同的线程运行。
        thread::sleep(Duration::from_millis(1));
    }
}

// 主线程打印了一些文本，新线程打印另一些文本