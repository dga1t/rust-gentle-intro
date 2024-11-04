
fn main() {
  let answer = 42;
  println!("Shalom {}", answer);
  
  assert_eq!(answer, 42);
  
  for i in 0..5 {
    if i % 2 == 0 {
      // println!("even {}", i)
    } else {
      // println!("odd {}", i)     
    }
    
    // this is also valid
    let even_odd = if i % 2 == 0 {"even"} else {"odd"};
    // println!("{} {}", even_odd, i)
  }
  
  let mut sum = 0;
  let mut sumFloat = 0.0;
  for i in 0..5 {
    sum += i;
    sumFloat += i as f64;
  }
  println!("sum is {}", sum);
  println!("sumFloat is {}", sumFloat);
}