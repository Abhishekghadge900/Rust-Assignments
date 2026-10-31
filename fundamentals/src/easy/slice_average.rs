/*
  Problem 11: Slice Average

  Write a function that takes a slice of f64 values and returns the arithmetic mean.
  If the slice is empty, return None.

  Run the tests for this problem with:
    cargo test --test slice_average_test 
*/

fn main(){
  let v = [1.0,2.0,3.0,4.0,5.0];
  let ans = average(&v);
  print!("{:?}",ans);
}

pub fn average(values: &[f64]) -> Option<f64> {
  let length = values.len();

  if length != 0 {
    let mean = values.iter().sum::<f64>() /(length as f64);
    Some(mean)
  }
  else{
    None
  }
}
