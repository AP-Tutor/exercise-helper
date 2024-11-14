#![procedural::magic_macro]
//!Implement two `tuple structs` named `Date` and `Hour`. The former takes `u8`, `u8` and `u16`  and the latter two `u8`
//!Implement a `BoxShipping` struct, with the fields `name: String`, `barcode: String`, shipment_date: Date` and `shipment_hour: Hour`
//!Make BoxShipping displayable both with :? the formatter as well as with a {} argument in println!.

use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
struct Date(u8, u8, u16);
#[derive(Debug)]
struct Hour(u8, u8);

#[derive(Debug)]
struct BoxShipping {
    name: String,
    barcode: String,
    shipment_date: Date,
    shipment_hour: Hour,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}/{:02}/{:04}", self.0, self.1, self.2)
    }
}

impl fmt::Display for Hour {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0, self.1)
    }
}

impl fmt::Display for BoxShipping {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name: {}, barcode: {}, date of shipment: {}, hour of shipment: {}",
            self.name, self.barcode, self.shipment_date, self.shipment_hour
        )
    }
}

#[runtest(1.0)]
/// checks if debug is implemented
fn test_debug() {
    let box_shipping = BoxShipping {
        name: String::from("box"),
        barcode: String::from("123456789"),
        shipment_date: Date(1, 1, 2020),
        shipment_hour: Hour(12, 0),
    };
    let _ = format!("Debug: \n{:?}", box_shipping);
}

#[runtest(1.0)]
/// checks if display is implemented
fn test_display() {
    let box_shipping = BoxShipping {
        name: String::from("box"),
        barcode: String::from("123456789"),
        shipment_date: Date(1, 1, 2020),
        shipment_hour: Hour(12, 0),
    };
    let _ =format!("Debug: \n{:?}", box_shipping);
}
