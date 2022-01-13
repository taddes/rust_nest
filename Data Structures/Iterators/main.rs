fn main() {
  let vec = vec![3, 2, 1];

  // if you use for loop to iterate, you have moved it, unless &
  for x in vec {
    println!("{}", x);
  }


}