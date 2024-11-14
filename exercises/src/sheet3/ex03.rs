#![procedural::magic_macro]
//! In Trento there is an automated car park with a camera that recognises the number plate of the car.
//! Your task is to associate the number plate with the owner of the car in order to track the price
//! for each car owner. Create a main with an appropriate data structure already initialised with
//! some data. Then create a new parking variabile of type Parking.
//!
//! Define a Parking struct with the following fields
//! - parked_cars: HashMap<String, String>
//! - remaining_spots: u32
//!
//! And then write these methods for Parking
//! - new: takes parked_cars and the number of maximum capacity to calculate the amount of the
//!   remaining spots and create a new Parking structs
//! - park_car: takes car_plate: String, owner: String and number of minutes: f32; returns Result<f32, &str>. Check if there are spots available
//!   and park the car inserting the car plate and the owner String inside the parked_cars.
//!   Then decrement by 1 remaining_spots and return a result with the number of minutes multiplicated
//!   by 0.25. Otherwise, with no sposts available return an error message.
//! - exit_parking: takes the car_plate: String and remove the car with the car_plate from parked_cars, adding 1
//!   to remaining spots and returning an Ok result. Otherwise if the car_plate wasn’t found return
//!   an Err result with an error message
//! - recognise_owner: takes a s: String with the car plate and
//!   a mutable reference hash: &mut HashMap<String, String. Returns an Option<&String> with,
//!   if exists, the name of the owner.

use std::collections::HashMap;

#[runtest(1.0, Parking::new)]
/// tests if Parking::new works
fn test_new_parking() {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    hash_map.insert("CX196SP".to_string(), "James".to_string());
    hash_map.insert("SASSARI".to_string(), "Silvio".to_string());

    let mut parking = Parking::new(hash_map, 3);
}

#[runtest(1.0, Parking::park_car)]
#[overwrite(Parking::new)]
/// tests if Parking::new works
fn test_park_car() {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    hash_map.insert("CX196SP".to_string(), "James".to_string());
    hash_map.insert("SASSARI".to_string(), "Silvio".to_string());

    let mut parking = Parking::new(hash_map, 3);
    assert_eq!(
        parking.park_car("ZZ121PS".to_string(), "Mario".to_string(), 10.),
        Ok(10. * 0.25)
    );
}

#[runtest(1.0, Parking::park_car)]
#[overwrite(Parking::new)]
/// tests if Parking::park_car Returns an error
fn test_park_car_when_full() {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    hash_map.insert("CX196SP".to_string(), "James".to_string());
    hash_map.insert("SASSARI".to_string(), "Silvio".to_string());

    let mut parking = Parking::new(hash_map, 2);
    assert!(parking
        .park_car("RT534LL".to_string(), "Luca".to_string(), 10.)
        .is_err());
}

#[runtest(1.0, Parking::exit_parking)]
#[overwrite(Parking::new)]
/// checks Parking::exit_parking on a real car
fn test_exit_parking_real() {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    hash_map.insert("CX196SP".to_string(), "James".to_string());
    hash_map.insert("SASSARI".to_string(), "Silvio".to_string());
    let mut parking = Parking::new(hash_map, 2);
    assert_eq!(parking.exit_parking("CX196SP".to_string()), Ok(()))
}

#[runtest(1.0, Parking::exit_parking)]
#[overwrite(Parking::new)]
/// checks Parking::exit_parking on a car that is not present in the parking lot
fn test_exit_parking_fake() {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    hash_map.insert("CX196SP".to_string(), "James".to_string());
    hash_map.insert("SASSARI".to_string(), "Silvio".to_string());
    let mut parking = Parking::new(hash_map, 2);
    assert!(parking.exit_parking("SASSARI2".to_string()).is_err())
}

#[runtest(1.0, Parking::recognise_owner)]
#[overwrite(Parking::new)]
/// checks Parking::recognise_owner when a car is inside parking lot
fn test_recognize_owner_real() {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    hash_map.insert("CX196SP".to_string(), "James".to_string());
    hash_map.insert("SASSARI".to_string(), "Silvio".to_string());
    let mut parking = Parking::new(hash_map, 2);
    assert_eq!(
        parking.recognise_owner("SASSARI".to_string()),
        Some("Silvio")
    )
}

#[runtest(1.0, Parking::recognise_owner)]
#[overwrite(Parking::new)]
/// checks Parking::recognise_owner when a car is not inside parking lot
fn test_recognize_owner_fake() {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    hash_map.insert("CX196SP".to_string(), "James".to_string());
    hash_map.insert("SASSARI".to_string(), "Silvio".to_string());
    let mut parking = Parking::new(hash_map, 2);
    assert_eq!(parking.recognise_owner("SASSARI2".to_string()), None)
}

/*
fn test() {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    hash_map.insert("CX196SP".to_string(), "James".to_string());
    hash_map.insert("SASSARI".to_string(), "Silvio".to_string());

    let mut parking = Parking::new(hash_map, 3);
    assert_eq!(
        parking.park_car("ZZ121PS".to_string(), "Mario".to_string(), 10.),
        Ok(10. * 0.25)
    );

    assert_eq!(
        parking.park_car("RT534LL".to_string(), "Luca".to_string(), 10.),
        Err("No more spots available")
    );

    assert_eq!(parking.exit_parking("NO".to_string()), Err("Car not found"));
    assert_eq!(parking.exit_parking("ZZ121PS".to_string()), Ok(()))
}*/

struct Parking {
    parked_cars: HashMap<String, String>,
    remaining_spots: u32,
}

impl Parking {
    pub fn new(parked_cars: HashMap<String, String>, max_capacity: u32) -> Self {
        let remaining_spots = max_capacity - parked_cars.len() as u32;
        Self {
            parked_cars,
            remaining_spots,
        }
    }

    pub fn park_car(
        &mut self,
        car_plate: String,
        owner: String,
        minutes: f32,
    ) -> Result<f32, &str> {
        if self.remaining_spots > 0 {
            self.parked_cars.insert(car_plate, owner);
            self.remaining_spots -= 1;
            Ok(minutes * 0.25)
        } else {
            Err("No more spots available")
        }
    }

    pub fn exit_parking(&mut self, car_plate: String) -> Result<(), &str> {
        if let Some(_) = self.parked_cars.remove(&car_plate) {
            self.remaining_spots += 1;
            Ok(())
        } else {
            Err("Car not found")
        }

        // match self.parked_cars.remove(&car_plate) {
        //     Some(_) => {
        //         self.remaining_spots += 1;
        //         Ok(())
        //     }
        //     None => Err("Car not found"),
        // }
    }

    pub fn recognise_owner(&self, s: String) -> Option<&str> {
        self.parked_cars.get(&s).map(|x| x.as_str())
    }
}
