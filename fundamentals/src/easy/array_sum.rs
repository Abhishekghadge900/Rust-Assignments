/*
  Problem 4: Array Sum

  Write a function that takes a reference to an array of exactly 5 i32 values and returns their sum.

  Run the tests for this problem with:
    cargo test --test array_sum_test
*/

fn main(){
  let mut arr :[i32;5] = [1;5];
  for i in 0..5{
    arr[i] = i as i32;
  }
  let ans = array_sum(&arr);
  print!("{}",ans);
}

pub fn array_sum(arr: &[i32; 5]) -> i32 {
    let sum = arr.iter().sum();
    sum
}
