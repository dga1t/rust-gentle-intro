// The code in string.rs where we extract the Russian greeting is not how it would be usually written. 

match multilingual.find('п') {
  Some(idx) => {
    let hi = &multilingual[idx..];
    println!("Russian hi {}", hi);
  },
  None => println!("couldn't find the greeting, Товарищ")
};

// But if you're not interested in failure here, then if let is your friend:
if let Some(idx) = multilingual.find('п') {
  println!("Russian hi {}", &multilingual[idx..]);
}
// This is convenient if you want to do a match and are only interested in one possible result.

// match can also operate like a C switch statement, and like other Rust constructs can return a value:
let text = match n {
  0 => "zero",
  1 => "one",
  2 => "two",
  _ => "many",
};

// Rust match statements can also match on ranges.
// In modern Rust, inclusive ranges are written as ..= instead of ...
let text = match n {
  0..=3 => "small",
  4..=6 => "medium",
  _ => "large",
};

