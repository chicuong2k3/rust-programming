
use std::fmt::Display;
fn main() {

    let user1 = User {
        username: String::from("cuong"),
        email: String::from("cuong@gmail.com"),
        sign_in_count: 1,
        active: true
    };

    let user2 = User {
        username: String::from("nguyen"),
        email: String::from("nguyen@gmail.com"),
        ..user1
    };

    println!("User1: {} - {} - {} - {}", user1.username, user1.email, user1.sign_in_count, user1.active);
    println!("User2: {} - {} - {} - {}", user2.username, user2.email, user2.sign_in_count, user2.active);

    let point = Point2D(3, 4);
    println!("Point: ({}, {})", point.0, point.1);

    println!("Point (debug): {:#?}", point);

    let ip_v4 = IPAddress {
        kind: IPAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let ip_v6 = IPAddress {
        kind: IPAddressKind::V6,
        address: String::from("::1"),
    };

    println!("IPv4: {} - {:?}", ip_v4.address, ip_v4.kind);
    println!("IPv6: {} - {:?}", ip_v6.address, ip_v6.kind);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Point2D(i32, i32);


impl Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

enum IPAddressKind {
    V4,
    V6
}

struct IPAddress {
    kind: IPAddressKind,
    address: String,
}