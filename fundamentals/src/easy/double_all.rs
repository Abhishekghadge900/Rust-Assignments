/*
  Problem 10: Mutable Reference — Double All

  Write a function that takes a mutable reference to a Vec<i32> and doubles every element in-place.
  The function should return nothing (unit type).

  Run the tests for this problem with:
    cargo test --test double_all_test
*/

fn main(){

  let mut vect = vec![1,4,3,7,6];
  let ans = double_all(&mut vect);
  print!("{:?}",ans);
}

pub fn double_all(values: &mut Vec<i32>)  { // -> &mut Vec<i32>
    values.iter_mut().for_each(|value| *value = *value * 2);
    //values
}
