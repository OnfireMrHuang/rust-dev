# Rust学习之旅

## 标量类型

1、整型

| 长度      | 有符号 |无符号|
| ----------- | ----------- |------|
| 8-bit|i8|u8|
| 16-bit|i16|u16|
| 32-bit|i32|u32|
| 64-bit|i64 |u64|
| 128-bit|i128|u128|
| arch|isize|usize|

1、浮点型

f32、f64

## 复合类型

1、元组类型

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

2、数组类型

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

## 函数

```rust
fn test(a :i32) -> i64 {
    a +1
}
```

```rust
fn main() {
    let number = 3;
    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }
}
```

## 枚举

```rust

// 第一种方式
enum IpAddrKind {
    V4,
    V6
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

// 第二种方式

enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"))

// 第三种方式
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127,0,0,1);
let loopback = IpAddr::V6(String::from("::1"))

// 第四种方式
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

// 第五种方式
enum Message {
    Quit,
    Move {x : i32, y : i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法
    }
}

let m = Message::Write(String::from("hello"));
m.call();

```

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;

```


```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

