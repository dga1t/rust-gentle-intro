
fn main() {
  let mut v = Vec::new();
  v.push(10);
  v.push(20);
  v.push(30);
  
  let first = v[0]; // will panic if out-of-range
  let maybe_first = v.get(0);
  
  println!("v is {:?}", v);
  println!("first is {}", first);
  println!("maybe_first is {:?}", maybe_first);
  
  // there is a very intimate relation between vectors and slices:
  dump(&v);
  let slice = &v[1..];
  println!("slice is {:?}", slice);
}

fn dump(arr: &[i32]) {
  println!("arr is {:?}", arr);
}