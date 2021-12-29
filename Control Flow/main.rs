fn if_stmt() {
  let temp = 25;
  if temp > 30 {
    println!("it is hot");
  }
  else if temp < 10 {
    println!("It is cold");
  }
  else {
    println!("Moderate")
  }

  let day = if temp > 20 {"sunny"} else {"cloudy"};

}

fn main() {
  if_stmt();
}