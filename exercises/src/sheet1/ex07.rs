#![procedural::magic_macro]
//! you will be provided with an hashmap with signature HashMap<String, f32>, which represent the association beetween furniture names and price.
//! write a function named get_furniture that takes an immutable reference to this hashmap, and the name of the forniture as a String. You should return the price or -1 if it is not available.
use std::collections::HashMap;

fn get_furniture(furniture: &HashMap<String, f32>, name: String) -> &f32 {
    furniture.get(name.as_str()).unwrap_or(&-1.0)
}

#[runtest(1.0, get_furniture)]
///tests get furniture
fn test() {
    let mut furniture: HashMap<String, f32> = HashMap::new();
    furniture.insert("Sofa".to_string(), 1200.);
    furniture.insert("Lamp".to_string(), 149.99);
    furniture.insert("Television".to_string(), 700.50);
    furniture.insert("Table".to_string(), 1499.99);

    assert_eq!(get_furniture(&furniture, "Sofa".to_string()), &1200.);
    assert_ne!(get_furniture(&furniture, "Lamp".to_string()), &1.99);
    assert_eq!(get_furniture(&furniture, "Null".to_string()), &-1.);
}
