#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main() {
  let a: u8 = 123; // u = unsigned, 8 bits, 0 - 255
  println!("a = {}", a);

  // u = unsigned, 0 to 2^N-1
  // i = signed,-2^(N-1) .. 2^(N-1) - 1,  -128 -> 127
  let mut b: i8 = 0; 
  println!("b = {}", b);
  b = 42;
  println!("b = {}", b);

  let c = 123456789; // i32 = 32 bits = 4 bytes
  println!("c = {}, c takes up {} bytes", c, mem::size_of_val(&c));
}