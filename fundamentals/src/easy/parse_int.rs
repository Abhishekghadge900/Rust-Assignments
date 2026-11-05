/*
  Problem 15: Parse Integer with Result

  Write a function that takes a &str and attempts to parse it into an i32.
  Return Ok(value) on success, or Err(String) with a descriptive error message on failure.

  Run the tests for this problem with:
    cargo test --test parse_int_test
*/

fn main(){
  let s = "10a";
  let ans = parse_int(&s);
  print!("{:?}",ans);
}

pub fn parse_int(s: &str) -> Result<i32, String> {
  s.parse::<i32>().map_err(|e| format!("failed to parse {} into an i32 :{}",s,e))
}
