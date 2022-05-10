// EXAMPLE 1: ->
enum IPAddrKind {
    V4,
    V8,
}

// EXAMPLE 2: ->
#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String)
}

// EXAMPLE 3: ->
enum Message {
    _Quit,
    _Move {x: i32, y: i32},
    Write(String),
    _ChangeColor(i32, i32, i32)
}

impl Message {
    fn _call(&self) {
        // method body would be defined here
    }
}

pub fn run() {
    /* Enums allow you to define a type by enumerating its possible variants. */

    // EXAMPLE 1: ->
    route(IPAddrKind::V4);
    route(IPAddrKind::V8);

    // EXAMPLE 2: ->
    let home = IPAddr::V4(String::from("127.0.0.1"));
    let _loopback = IPAddr::V6(String::from("::1"));

    println!("{:?}", home);

    // EXAMPLE 3: ->
    let _top = Message::Write(String::from("Hello"));
}

fn route(_ip_kind: IPAddrKind) {

}
