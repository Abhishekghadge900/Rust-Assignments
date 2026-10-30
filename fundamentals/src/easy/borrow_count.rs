/*
  Problem 9: Borrow and Count

  Write a function that takes an immutable reference to a Vec<i32> and returns the count of elements greater than a given threshold.
  This exercises borrowing without taking ownership.

  Run the tests for this problem with:
    cargo test --test borrow_count_test
*/

fn main(){
  let ve = vec![1,4,7,2,8,9,0,3,5,2,7];
  let g_val = 5;
  let ans = count_above(&ve, g_val);
  print!("{}",ans);
}

pub fn count_above(values: &Vec<i32>, threshold: i32) -> usize {
    let mut count = 0;
    for v in values{
      if *v > threshold{
        count += 1;
      }
    }
    count 
}
