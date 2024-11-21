
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
  
  // more about vectors..
  // vectors compare with each other and with slices by value
  let mut v1 = vec![10, 20, 30, 40];
  v1.pop();  
  let mut v2 = Vec::new();
  v2.push(10);
  v2.push(20);
  v2.push(30);
  assert_eq!(v1, v2);
  
  v2.extend(0..2);
  assert_eq!(v2, &[10, 20, 30, 0, 1]);
  
  // vectors can be sorted, and then duplicates can be removed - these operate in-place on the vector
  // (if you want to make a copy first, use clone)
  let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
  v1.sort();
  v1.dedup();
  assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
}

fn dump(arr: &[i32]) {
  println!("arr is {:?}", arr);
}