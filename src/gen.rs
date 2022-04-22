static CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*()_";

pub fn generate_passcode(len: usize) -> String {
  let chars_length = CHARS.len();
  return CHARS.chars().nth(len % chars_length).unwrap().to_string();
}