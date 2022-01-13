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

    let _1_5:HashSet<_> = (1..=5).collect();
    let _6_10:HashSet<_> = (6..=10).collect();
    let _1_10:HashSet<_> = (1..=10).collect();
    let _2_8:HashSet<_> = (2..=8).collect();

    // subset - are the elements of one collection contained in the other
    println!(
      "is {:?} a subset of {:?} {}",
      _2_8, _1_10,
      _2_8.is_subset(&_1_10)
    );


}
