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

  let _day = if temp > 20 {"sunny"} else {"cloudy"};

  println!("It is {}",
if temp > 20 {
  if temp > 30 {"Very Hot"} else {"HOT"}
} else if temp < 10 {"cold"} else {"ok"}) 

}

fn while_loop() {
  let mut x = 1;

  while x < 1000 {
    x *= 2;
    println!("x = {}", x);
  }
}

fn for_loop() {
  for x in 1..11 {
    if x == 3 { continue; }
    println!("{}", x);
  }

  for (idx, val) in (30..40).enumerate() {
    println!("{} is at index {}", val, idx)
  }
}

fn match_cases() {
  let country_code = 44;

  let country = match country_code {
    44 => "UK",
    1 => "US",
    31 => "FI",
    1..=1000 => "unknown",
    _ => "invalid"
  };

  println!("The country with code {} is {}", country_code, country);
}

fn main() {
  if_stmt();
  while_loop();
  for_loop();
  match_cases();
}