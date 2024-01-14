enum IpAddr {
    v4(String),
    v6(String),
}

fn main() {
    let four = IpAddr::v4(String::from("127.0.0.1"));
    let six = IpAddr::v6(String::from("::1");
}
