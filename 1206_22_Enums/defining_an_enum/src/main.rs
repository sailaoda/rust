fn main() {
    println!("Hello, world!");
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

// 枚举优势一： 可以直接将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了。

enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

// 枚举优势二： 每个成员可以处理不同类型和数量的数据。
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

// 可以将任意类型的数据放入枚举成员中： 例如字符串、数字类型或者结构体。甚至可以包含另一个枚举！
