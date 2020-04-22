enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Writer(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //TODO
        println!("{:#?}", self)
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Writer(String::from("hello"));

    m.call();
}

fn route(ip_kind: IpAddrKind) {}
