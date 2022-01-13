
use std::thread;
use std::time;
fn strings() {
  // Vector of utf-8 characters
  // &str = string slice
  let s:&'static str = "Hello there!";
  // can't assign slices or reassign, or get indecies since they are utf-8 bytes

  for c in s.chars() { // chars().rev()
    println!("{}", c);
  }

  let a = "Hello World!";
  println!("{}", a);

  let b = a.chars();
  println!("{:?}", b);
  println!("{}", a);

  // safe way to get a character
  if let Some(first_char) = s.chars().nth(0) {
    println!("First letter {}", first_char);
  }

  // String class
  // Heap allocated, utf-8 sequence
  let mut letters = String::new();
  let mut a = 'a' as u8;
  while a <= ('z' as u8) {
    letters.push(a as char);
    letters.push_str(",");
    a += 1;
  }
  println!("{}", letters);

  // &str <> String
  // let u:&str = &letters; // deref conversion

  // concat
  // String + str works
  // let z = letters + &letters;

  let mut abc = String::from("hello world");
  let mut def = "hey world".to_string();

  abc.remove(0);
  abc.push_str("!!!");
  println!("{}", abc);
  abc = abc.replace("ello", "goodbye");
  println!("{}", abc);
}

fn formatter() {

}

fn main() {
  strings();
}