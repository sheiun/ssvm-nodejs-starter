use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate(s_x: &str, s_y: &str, op: &str) -> String {
  let x: i32 = s_x.parse().unwrap();
  let y: i32 = s_y.parse().unwrap();
  let r: i32 = match op {
    "+" => add(x, y),
    "-" => min(x, y),
    "*" => mul(x, y),
    "/" => div(x, y),
    _ => 0
  };
  return String::from("Answer is ") + &r.to_string();
}

pub fn add(x: i32, y: i32) -> i32 {
  return x + y;
}

pub fn min(x: i32, y: i32) -> i32 {
  return x - y;
}

pub fn mul(x: i32, y: i32) -> i32 {
  return x * y;
}

pub fn div(x: i32, y: i32) -> i32 {
  return x / y;
}
