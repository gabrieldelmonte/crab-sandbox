#[derive(Debug)]
enum IPAddressVersion {
    V4,
    V6,
}

#[derive(Debug)]
struct IPAddressStruct {
    version: IPAddressVersion,
    address: String,
}

#[derive(Debug)]
enum IPAddressEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("Using the struct version of IPAddress:");
    let mut ip_struct = IPAddressStruct {
        version: IPAddressVersion::V4,
        address: String::from("192.168.1.1"),
    };
    println!("Struct IP Address: {:#?}", ip_struct);
    println!();
    
    ip_struct.version = IPAddressVersion::V6;
    ip_struct.address = String::from("10.0.0.1");
    println!("Updated Struct IP Address: {:#?}", ip_struct);
    println!();

    println!("Using the enum version of IPAddress:");
    let mut ip_enum = IPAddressEnum::V4(192, 168, 1, 1);
    println!("Enum IP Address: {:#?}", ip_enum);
    println!();

    ip_enum = IPAddressEnum::V6(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
    println!("Updated Enum IP Address: {:#?}", ip_enum);
    println!();
}
