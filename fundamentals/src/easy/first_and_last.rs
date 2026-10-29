/*
  Problem 7: First and Last

  Write a function that takes a slice of i32 values and returns a tuple of the first and last elements.
  If the slice is empty, return None.

  Run the tests for this problem with:
    cargo test --test first_and_last_test
*/

fn main(){
  let mut arr: Vec<i32> = Vec::new();
  for a in 0..5{
    arr.push(a);
  }
  let ans = first_and_last(&arr);
  print!("{:?}",ans);
}

pub fn first_and_last(slice: &[i32]) -> Option<(i32, i32)> {
    let length = slice.len();
    if length == 0 {
      return None
    }
    else{
      Some((slice[0],slice[length-1]))
    }
}
