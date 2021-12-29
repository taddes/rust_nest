#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

struct Point {
  x: f64,
  y: f64
}

struct Line {
  start: Point,
  end: Point
}

fn structures() {
  let p = Point  {x: 3.0, y: 4.0 };
  println!("point p is at {}, {}", p.x, p.y);

  let p2 = Point { x: 5.0, y: 10.0};
  let myline = Line { start:p, end:p2 };
  println!("Line x coords {}, {}", myline.start.x, myline.end.x);
}

fn enumeration() {
  enum Color {
    Red, 
    Green, 
    Blue,
    RGBColor(u8, u8, u8)
  }

  let c:Color = Color::Red;

  match c {
    Color::Red => println!("r"),
    Color::Green => println!("g"),
    Color::Blue => println!("b"),
    Color::RGBColor(0, 0, 0) => println!("Black"),
    Color::RGBColor(r, g, b) => println!("rgb {}, {}, {}", r, g, b),
    _ => println!("Something Else")
  }

}

fn main() {
  // structures();
  enumeration();
}