union IntOrFloat 
{
  i: i32,
  f: f32
}

fn process_value(iof: IntOrFloat) {
  unsafe {
    match iof {
      IntOrFloat { i: 42} => {
        println!("meaning of life value");
      }
      IntOrFloat { f } => {
        println!("Value is {}", f)
      }
    }
  }
}

fn main() {
  let mut iof = IntOrFloat { i: 123 };
  iof.i = 234;

  let value = unsafe { iof.i };


  process_value(IntOrFloat{i:42});
}