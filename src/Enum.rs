#[derive(Debug)]
enum TypeIpAddress {
    V4,
    V6,
}

#[derive(Debug)]
enum TypeIpAddress2 {
    V4(String),
    V42(u8, u8, u8, u8),
}

#[derive(Debug)]
struct _IpAddress {
    typeAddress: TypeIpAddress,
    address: String,
}

#[derive(Debug)]
struct _IpAddress2 {
    typeAddress: TypeIpAddress,
    address: String,
}

fn main() {
    let four: TypeIpAddress = TypeIpAddress::V4;
    let six: TypeIpAddress = TypeIpAddress::V6;
    println!("{:?}", four);
    println!("{:?}", six);

    let localhost: _IpAddress = _IpAddress {
        typeAddress: TypeIpAddress::V4,
        address: String::from("127.0.0.1"),
    };

    println!("{:#?}", localhost);

    let localhost2 = TypeIpAddress2::V4(String::from("127.0.0.1"));
    let localhost2_2 = TypeIpAddress2::V42(127, 0, 0, 1);
    println!("{:#?}", localhost2);
    println!("{:#?}", localhost2_2);
}
