fn main() {
  let amount = interpreter("100", "1111111111");

  println!("{}", amount);
}

fn interpreter(tape: &str, data: &str) -> String {
  // your solution here
  let len = data.len();
  if len == 0 {
    return String::from("");
  }

  let mut res = String::new();
  let mut chars = data.chars();
  let mut cur_char = chars.next();

  let mut tape_chars = tape.chars();
  let mut cur_op = tape_chars.next();

  while cur_char.is_some() {
    match cur_op {
      Some('0') => {
        res.push(cur_char.unwrap());
        cur_op = tape_chars.next();
        cur_char = chars.next();
      }
      Some('1') => {
        match cur_char {
          Some('1') => {
            cur_char = Some('0');
          }
          Some('0') => {
            cur_char = Some('1');
          }
          _ => {
            panic!("Unknow character at data");
          }
        }
        cur_op = tape_chars.next();
      }
      _ => {
        tape_chars = tape.chars();
        cur_op = tape_chars.next();
      }
    }
  }

  res
}
// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn basic_tests() {
  // flip every bit
  assert_eq!(interpreter("10", "1010101"), "0101010");
  // flip every other bit
  assert_eq!(interpreter("100", "1111111111"), "0101010101");
}