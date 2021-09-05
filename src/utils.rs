use std::env;

pub fn env_get(x: String) -> String {
  for (key, value) in env::vars() {
    if key == x {
      return value;
    }
  }
  return "".to_owned();
}
