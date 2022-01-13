use std::collections::HashMap;

fn main() {
  let mut shapes = HashMap::new();
  shapes.insert(String::from("triangle"), 2);
  shapes.insert(String::from("square"), 4);

  println!("a square has {} sides", shapes["square"]);
  for (key, value) in &shapes {
    println!("{} {}", key, value);
  }

  shapes.entry("circle".into()).or_insert(1);

  println!("{:?}", shapes);
}