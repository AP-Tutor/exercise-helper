#![procedural::magic_macro]
//! For the following examples, decide which of the composite data structures is better (enum or structs). Then implent them
//!  - you are Rick, a car shop owner, and you have to choose the fuel of your car between Diesel, Gasoline, LPG, Methane and Electric
//!  - you have to program the recognition of the IP version of a router. Remember that IPv4 is formatted with 4 group of 3 integer values (from 0 to 255), IPv6 is instead formatted with 8 groups of 4 **hexadecimal** values.
//!  - you have to track points in a 3 dimensional space, with the f64 values for each dimension

enum Fuel {
    Diesel,
    Gasoline,
    LPG,
    Methane,
    Electricity,
}

enum Ip {
    Ipv4([u8; 4]),
    Ipv6([u16; 8]),
}

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[runtest(1.0, Ip)]
/// checks if Ip is implemented correctly
fn test_ip() {
    // visualization (not required, only for testing)
    impl ToString for Ip {
        fn to_string(&self) -> String {
            match &self {
                Ip::Ipv4(v) => format!("{}.{}.{}.{}", v[0], v[1], v[2], v[3]),
                Ip::Ipv6(v) => format!(
                    "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
                    v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]
                ),
            }
        }
    }

    assert_eq!(Ip::Ipv4([192, 168, 1, 5]).to_string(), "192.168.1.5");
    assert_eq!(
        Ip::Ipv6([0x2001, 0xdb8, 0x3333, 0x4444, 0x5555, 0x6666, 0x7777, 0x8888]).to_string(),
        "2001:db8:3333:4444:5555:6666:7777:8888"
    )
}

#[runtest(1.0, Point)]
/// checks if Point is implemented correctly
fn test_point() {
    let p = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}
#[runtest(1.0, Fuel)]
/// checks if Fuel is implemented correctly
fn test_fuel() {
    let m = Fuel::Diesel;
    match m {
        Fuel::Diesel => {}
        Fuel::Gasoline => {}
        Fuel::LPG => {}
        Fuel::Methane => {}
        Fuel::Electricity => {}
    }
}
