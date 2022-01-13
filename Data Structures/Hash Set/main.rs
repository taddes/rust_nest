#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::thread;
use std::time;
use std::collections::HashSet;

fn main() {
  let mut greeks = HashSet::new();
  greeks.insert("gamma");
  greeks.insert("delta");
  greeks.insert("omicron");

  let added_vega = greeks.insert("vega");

    // Order of insertion does not necessarily match output
    println!("{:?}", greeks);
}