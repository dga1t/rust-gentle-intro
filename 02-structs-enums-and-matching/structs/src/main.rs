// The directive makes the compiler generate a Debug implementation, which is very helpful.
// It's good practice to do this for your structs, so they can be printed out (or written as a string using format!).
// (Doing so by default would be very un-Rustlike.)
#[derive(Debug)]
struct Person {
  first_name: String,
  last_name: String
}

impl Person {
  fn new(first: &str, name: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: name.to_string()
    }
  }
  // The self is used explicitly and is passed as a reference. (You can think of &self as short for self: &Person.)
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }
  // The keyword Self refers to the struct type - you can mentally substitute Person for Self here:
  fn copy(&self) -> Self {
    Self::new(&self.first_name, &self.last_name)
  }
  // Methods may allow the data to be modified using a mutable self argument:
  fn set_first_name(&mut self, name: &str) {
    self.first_name = name.to_string();
  }
  // And the data will move into the method when a plain self argument is used:
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

// Usually structs contain values, but often they also need to contain references.
// Say we want to put a string slice, not a string value, in a struct.
#[derive(Debug)]
struct A {
  s: &'static str
}

// This can also be used to specify a string slice that is returned from a function:
fn how(i: u32) -> &'static str {
  match i {
  0 => "none",
  1 => "one",
  _ => "many"
  }
}

// However we can specify that the lifetime of the reference is at least as long as that of the struct itself.
#[derive(Debug)]
struct B <'a> {
  s: &'a str
}

fn makes_b() -> B<'static> {
  let string = "I'm a little string".to_string();
  B { s: &string }
}

fn main() {
  // To make initilization less clumsy we can move construction of Person into its own function.
  // let p = Person {
  //   first_name: "John".to_string(),
  //   last_name: "Smith".to_string()
  // };
  
  let mut p = Person::new("Jhon", "Cena");
  println!("{:?}", p);
  println!("fullname {}", p.full_name());
  p.set_first_name("Jane");
  println!("{:?}", p);
  println!("{:?}", p.to_tuple());
  // p has now moved.
  
  // Now, string slices borrow from string literals like "hello" or from String values.
  // String literals exist for the duration of the whole program, which is called the 'static' lifetime.
  let a = A { s: "hello dammit" };
  println!("{:?}", a);
  
  // Lifetimes are conventionally called 'a','b',etc but you could just as well called it 'me' here.
  // After this point, our b struct and the s string are bound by a strict contract: b borrows from s, and cannot outlive it.
  let s = "I'm a little string".to_string();
  let b = B { s: &s };
  println!("{:?}", b);
  // You can usefully think of lifetime parameters as being part of the type of a value.
}