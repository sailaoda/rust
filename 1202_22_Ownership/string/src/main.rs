// 在 C++ 中，这种 item 在生命周期结束时释放资源的模式有时被称作 资源获取即初始化（Resource Acquisition Is Initialization (RAII)）
fn main() {
    let mut s = String::from("hello1111");

    {
        let s = String::from("hello");
    } // drop函数，在结尾的}自动调用drop

    s.push_str(",world!");

    println!("{}", s);
}
