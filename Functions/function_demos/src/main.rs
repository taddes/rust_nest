fn main() {
    functions();
    let p: i32 = product(5,5);
    println!("{}", p);
}

fn print_value(x: i32) {
  println!("value = {}", x);
}
fn increase(x: &mut i32) {
  *x += 1;
}

fn functions() {
  let mut z: i32 = 12;
  increase(&mut z);
  print_value(z);
  println!("{}", z);

}

fn product(x:i32, y:i32) -> i32 {
  x * y
}

struct Point {
  x: f64,
  y: f64
}

struct Line {
  start: Point,
  end: Point
}

impl Line {
  fn len(&self) -> f64 {
    let dx = self.start.x - self.end.x;
    let dy = self.start.y - self.end.y;
    (dx*dx + dy*dy).sqrt()
  }
}

fn methods() {
  let p = Point { x: 3.0, y: 4.2};
  let 2p = Point { x: 5.0, y: 10.0};

}