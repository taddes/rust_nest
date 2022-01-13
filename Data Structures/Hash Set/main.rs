#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::collections::HashSet;
use std::thread;
use std::time;

fn main() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    greeks.insert("omicron");

    // Order of insertion does not necessarily match output
    println!("{:?}", greeks);

    // While inserting, this will result in a bool if inserted successfully
    // If not, False will return (in the case of a duplicate)
    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("added")
    }

    // Order of insertion does not necessarily match output
    println!("{:?}", greeks);

    let mut val_to_check = "kappa";
    if !greeks.contains(&val_to_check) {
        println!("{} is not in collection", val_to_check);
    } else {
        println!("{} is in collection", val_to_check);
    }

    // Order of insertion does not necessarily match output
    println!("{:?}", greeks);

    let removed = greeks.remove("delta");
    println!("{:?}", removed);

    if removed {
      println!("delta was removed")
    }
}
