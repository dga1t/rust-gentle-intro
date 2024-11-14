
fn main() {
  let res = sqr(2.0);
  println!("square is {}", res);
  
  let i = 10;
  let res1 = by_ref(&i);
  let res2 = by_ref(&41);
  println!("{} {}", res1, res2);
  
  let mut res3 = 0.0;
  modifies(&mut res3);
  println!("res is {}", res3);
}

fn sqr(x: f64) -> f64 {
  // return x * x;
  // popular way to write without return
  x * x
}

// a few more examples of this no-return expression style:

fn abs(x: f64) -> f64 {
  if x > 0.0 {
    x
  } else {
    -x
  }
}

fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
  if x < x1 {
    x1
  } else if x > x2 {
    x2
  } else {
    x
  }
}

// values can also be passed by reference:

fn by_ref(x: &i32) -> i32 {
  *x + 1
}

// what if you want a function to modify one of its arguments? Enter mutable references:

fn modifies (x: &mut f64) {
  *x = 1.0;
}