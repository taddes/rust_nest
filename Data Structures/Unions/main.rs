union IntOrFloat 
{
  i: i32,
  f: f32
}

fn main() {
  let mut iof = IntOrFloat { i: 123 };
  iof.i = 234;

  let value = unsafe { iof.i };
}