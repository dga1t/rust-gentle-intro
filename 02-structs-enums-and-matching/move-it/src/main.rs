
fn main() {
  let s1 = "hello dolly".to_string();
  let s2 = s1;
  // gives error[E0382]: use of moved value: `s1`
  println!("s1 {}", s1);
  
  // Rust has different behaviour than other languages. In a language where variables are always references (like Java or Python),
  // s2 becomes yet another reference to the string object referenced by s1. In C++, s1 is a value, and it is copied to s2.
  // But Rust moves the value. It doesn't see strings as copyable ("does not implement the Copy trait").
  
  // Re-writing with a function call reveals exactly the same error:
  dump1(s1);
  println!("s1 {}", s1); // <---error: 'value used here after move'
  
  // So altogether the best way to declare that function is:
  dump2(&s1);
  println!("s1 {}", s1);
  
  // And then both dump2(&s1) and dump2("hello world") work properly.
  // (Here Deref coercion kicks in and Rust will convert &String to &str for you.)
  
  // To summarise, assignment of a non-Copy value moves the value from one location to another.
  // Otherwise, Rust would be forced to implicitly do a copy and break its promise to make allocations explicit.
}

fn dump1(s: String) {
  println!("{}", s);
}

fn dump2(s: &str) {
  println!("{}", s);
}



