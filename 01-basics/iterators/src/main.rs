
fn main() {
  let mut iter = 0..3;
  assert_eq!(iter.next(), Some(0));
  assert_eq!(iter.next(), Some(1));
  assert_eq!(iter.next(), Some(2));
  assert_eq!(iter.next(), None);
  
  let arr = [10, 20, 30];
  // fails without .iter() method
  for i in arr.iter() {
    println!("{}", i);
  }
  // slices are converted implicitly to iterators..
  let slice = &arr;
  for i in slice {
    println!("{}", i);
  }
  
  // idiomatic pro-level way of doing the sum
  let sum: i32 = (0..5).sum();
  println!("sum was {}", sum);
  let sum: i64 = [10, 20, 30].iter().sum();
  println!("sum was {}", sum);
  
  // the windows method gives you an iterator of slices - overlapping windows of values!
  let ints = [1, 2, 3, 4, 5];
  let slice = &ints;
  for s in slice.windows(2) {
    println!("window {:?}", s);
  }
  // or chuncks
  for s in slice.chunks(2) {
    println!("chunks {:?}", s);
  }
}