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
  fn to_tuple(self) -> (String,String) {
    (self.first_name, self.last_name)
  }
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
}