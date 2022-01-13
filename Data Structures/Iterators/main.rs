fn main() {
  let vec = vec![3, 2, 1];

  // if you use for loop to iterate, you have moved it, unless &
  for x in &vec {
    println!("{}", *x);
  }

  println!("{:?}", vec);

  // iter gives you immutavble references
  for x in vec.iter() {
    println!("{}", x); // no star, rust figures it out
  }

  // mutable iterator
  let mut vec2 = vec![6, 5, 4];

  for x in vec2.iter_mut() {
    println!("{}", x);
  }
  println!("{:?}", vec2);

  // print in reverse
  for y in vec2.iter().rev() {
    println!("in reverse {}", y);
  }

  // transform elements from one collection into an iterator
  // when you don't care about the collection but need the elements
  let it = vec.into_iter();
  println!("{:?}", it);
  //  vec2.extend(vec); // behind scenes, extend calls into_iter
}