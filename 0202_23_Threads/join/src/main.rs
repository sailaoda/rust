use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 放到中间则等上面执行完再 执行下面的主线程。影响线程是否同时运行
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 通过调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。
    // 阻塞（Blocking）线程意味着阻止该线程执行工作或退出。
    handle.join().unwrap();
}