enum IpAddrKind {
    V4(String),
    V6(String)
}

fn main() {
    
    let ip_kind = IpAddrKind::V4(String::from("teste"));
    let another_ip_kind = IpAddrKind::V6(String::from("te"));

    route(ip_kind);
    route(another_ip_kind);
}

fn route(ip: IpAddrKind) {
    // if ip == IpAddrKind::V4 {
    //     println!("vi foru");
    // } else if ip == IpAddrKind::V6 {
    //     println!("vi sicsu")
    // }
}
