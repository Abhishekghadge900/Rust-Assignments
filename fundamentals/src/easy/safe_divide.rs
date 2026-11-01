/*
  Problem 14: Safe Division with Option

  Write a function that divides two f64 numbers.
  Return None if the divisor is zero, otherwise return Some(result).

  Run the tests for this problem with:
    cargo test --test safe_divide_test
*/

fn main(){
  let a = 10.0;
  let b = 5.0;
  let ans = safe_divide(a, b);
  print!("{:?}",ans);
}

pub fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0 as f64{
      None
    }
    else{
      Some(a/b)
    }
}
