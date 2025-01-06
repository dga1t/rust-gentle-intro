use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

fn read_to_string(filename: &str) -> Result<String,io::Error> {
  let mut file = match File::open(&filename) {
    Ok(f) => f,
    Err(e) => return Err(e),
  };
  let mut text = String::new();
  match file.read_to_string(&mut text) {
    Ok(_) => Ok(text),
    Err(e) => Err(e),
  }
}

fn main() {
  // This version of the file reading function does not crash. (skipped the first crushing version)
  // It returns a Result and it is the caller who must decide how to handle the error.
  let file = env::args().nth(1).expect("please supply a filename");
  let text = read_to_string(&file).expect("bad file man!");
  println!("file had {} bytes", text.len());
  
  // Just an example of Result<Ok,Error>
  println!("{:?}",good_or_bad(true));   //Ok(42)
  println!("{:?}",good_or_bad(false));  //Err("bad")
  match good_or_bad(true) {
    Ok(n) => println!("Cool, I got {}",n),
    Err(e) => println!("Huh, I just got {}",e)
  }
}

// The actual 'error' type is arbitrary - a lot of people use strings until they are comfortable with Rust error types.
fn good_or_bad(good: bool) -> Result<i32,String> {
  if good {
    Ok(42)
  } else {
    Err("bad".to_string())
  }
}
