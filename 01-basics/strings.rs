
fn main() {
  let text = "hello dolly";  // the string slice
  let s = text.to_string();  // it's now an allocated string

  dump(text);
  dump(&s);
  
  // like a vector, you can push a character and pop one off the end of String:
  let mut s = String::new();
  // initially empty!
  s.push('H');
  s.push_str("ello");
  s.push(' ');
  s += "World!";  // short for `push_str`
  s.pop();        // remove the last char

  assert_eq!(s, "Hello World");
  
  // you can convert many types to strings using to_string (if you can display them with '{}' then they can be converted)
  let arr = array_to_str(&[10, 20, 30]);
  let res = format!("hello {}", arr);
  
  assert_eq!(res, "hello [10,20,30]");
  
  // the notation used for slices works with strings as well:
  let text = "static";
  let string = "dynamic".to_string();

  let text_s = &text[1..];
  let string_s = &string[2..4];

  println!("slices {:?} {:?}", text_s, string_s);
  
  // but, you cannot index strings!
  // this is because they use the One True Encoding, UTF-8, where a 'character' may be a number of bytes
  let multilingual = "Hi! ¡Hola! привет!";
  for ch in multilingual.chars() {
    print!("'{}' ", ch);
  }
  println!("");
  println!("len {}", multilingual.len());
  println!("count {}", multilingual.chars().count());

  let maybe = multilingual.find('п');
  if maybe.is_some() {
    let hi = &multilingual[maybe.unwrap()..];
    println!("Russian hi {}", hi);
  }
  
  // the string split_whitespace method returns an iterator, and we then choose what to do with it
  // collect is very general and so needs some clues about what it is collecting - hence the explicit type
  let text = "the red fox and the lazy dog";
  let words: Vec<&str> = text.split_whitespace().collect();
  // have a look at this cute one-liner; we get an iterator over the chars, and only take those characters which are not space
  let stripped: String = text.chars().filter(|ch| ! ch.is_whitespace()).collect();
}

fn dump(s: &str) {
  println!("str '{}'", s);
}

fn array_to_str(arr: &[i32]) -> String {
  let mut res = '['.to_string();
  for v in arr {
    res += &v.to_string();
    res.push(',');
  }
  res.pop();
  res.push(']');
  res
}
