
fn main() {
  let arr = [10, 20, 30, 40];
  let first = arr[0];
  println!("first {}", first);
  
  for i in 1..4 {
    println!("[{}] = {}", i,arr[i]);
  }
  println!("length {}", arr.len());
  
  let res = sum(&arr);
  println!("sum {}", res);
  
  // --- slicing and dicing ---
  let ints = [1, 2, 3];
  let floats = [1.1, 2.1, 3.1];
  let strings = ["hello", "world"];
  let ints_ints = [[1, 2], [10, 20]];
  println!("ints {:?}", ints);
  println!("floats {:?}", floats);
  println!("strings {:?}", strings);
  println!("ints_ints {:?}", ints_ints);
  
  let ints = [1, 2, 3, 4, 5];
  let slice1 = &ints[0..2];
  let slice2 = &ints[1..];  // open range!

  println!("ints {:?}", ints);
  println!("slice1 {:?}", slice1);
  println!("slice2 {:?}", slice2);
  
  // --- optional values ---
  let ints = [1, 2, 3, 4, 5];
  let slice = &ints;
  let first = slice.get(0);
  let last = slice.get(5);

  println!("first {:?}", first);
  println!("last {:?}", last);
  
  // the Option type has some useful methods:
  println!("first {} {}", first.is_some(), first.is_none());
  println!("last {} {}", last.is_some(), last.is_none());
  println!("first value {}", first.unwrap());
  
  let maybe_last = slice.get(5);
  let last = if maybe_last.is_some() {
    *maybe_last.unwrap()
  } else {
    -1
  };
  
  let last = *slice.get(5).unwrap_or(&-1);
}

// read as: slice of i32 
fn sum(values: &[i32]) -> i32 {
  let mut res = 0;
  for i in 0..values.len() {
    res += values[i]
  }
  res
}