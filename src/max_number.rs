fn main() {
  let amount = max_number(213);
  println!("{:?}", amount);
}

fn max_number(n: u32) -> u32 {
  let mut v = n
    .to_string()
    .split("")
    .filter(|&d| d != "")
    .map(|d| {
      d.parse::<u32>().unwrap()
    })
    .collect::<Vec<u32>>();

  v.sort();
  v.reverse();
  v.into_iter().fold(0 as u32, |acc, d| acc * 10 + d)
}

#[test]
fn basic_tests() {
  assert_eq!(max_number(213), 321);
  assert_eq!(max_number(7389), 9873);
  assert_eq!(max_number(63729), 97632);
  assert_eq!(max_number(566797), 977665);
  assert_eq!(max_number(17693284), 98764321);
}
