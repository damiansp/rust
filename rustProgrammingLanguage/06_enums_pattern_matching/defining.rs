fn main() {
    let four = IPAddressTypeSimple::V4;
    let six = IPAddressTypeSimple::V6;
    //let home = IPAddressVerbose {
    //    kind: IPAddressTypeSimple::V4, address: String::from("127.0.0.1")}
    //let loopback = IPAddressVerbose {
    //    kind: IPAddressTypeSimple::V6, address: String::from("::1")}
    let  home = IPAddress::V4(String::from("127.0.0.1"));
    let loopback = IPAddress::V6(String::from("::1"));
}

enum IPAddressTypeSimple {
    V4,
    V6
}


enum IPAddress {
    V4(String),
    V6(String)
}


fn route(ip_type: IPAddressTypeSimple) { /* */ }


struct IPAddressVerbose {
    kind: IPAddressTypeSimple,
    address: String
}


