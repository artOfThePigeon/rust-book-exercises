fn main() {
    // this syntax is valid but can be more concise using enums
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
        address: String::from("127.0.01"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // the following accomplishes the same as the above but more concise. each variant can have different types and amounts of associated data
    enum IpAddrConcise {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    // we attach data to each variant of the enum directly.
    // the name of each enum variant defined becomes a function that constructs an instance of the enum.
    let home_concise = IpAddrConcise::V4(127, 0, 0, 1);
    let loopback_concise = IpAddrConcise::V6(String::from("::1"));

    // enums can have a wide variety of types
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // the following structs could hold the same data that the preceding enum variants hold:
    struct QuitMessage;
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
    // ^^ But if we used the different structs, each of which has its own type, we couldn’t as easily define
    // a function to take any of these kinds of messages as we could with the Message enum , which is a single type.

    //  we’re also able to define methods on enums. Here’s a method named call that we could define on our Message enum:
    impl Message {
        fn call(&self) {
            // define method body here
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

}
