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

  println!("It is {}",
if temp > 20 {
  if temp > 30 {"Very Hot"} else {"HOT"}
} else if temp < 10 {"cold"} else {"ok"}) 

}

fn main() {
  if_stmt();
}