use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> Result<String, Error> {
  let mut string = String::new();
      // Access a file at a specified path
    // ---------------------------------
    // TODO #1:
    // - Pass variable to `file` variable on success, or
    // - Return from function early if there's an error
    let mut file: File = match File::open(path) {
      Ok(file_handle) => todo!("Pass variable to file variable on success"), 
      Err(io_error) => todo!("Return the function early if error")
    };

}


fn main() {
    println!("Hello, world!");
}
