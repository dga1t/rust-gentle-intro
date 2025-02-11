
fn main() {
  for arg in std::env::args() {
    println!("'{}'", arg);
  }
  
  // would it have been better to return a Vec?
  let args: Vec<String> = std::env::args().skip(1).collect(); 
  if args.len() > 0 {}
  
  // a more Rust-y approach to reading a single argument (together with parsing an integer value):
  let first = std::env::args().nth(1).expect("please supply an argument");
  let n: i32 = first.parse().expect("not an integer!");
}