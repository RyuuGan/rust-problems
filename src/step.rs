use std::collections::HashMap;

fn main() {
  let amount = step(2,10000000,11000000);
  println!("{:?}", amount);
}

fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
  let mut map: HashMap<u64, u64> = HashMap::new();
  for prime in m..(n + 1) {
    if is_prime(prime) {
      if map.contains_key(&(prime - g as u64)) {
        return Some(((prime - g as u64), prime));
      }
      map.insert(prime, prime);
    }
  }

  None
}

fn is_prime(num: u64) -> bool {
  if num % 2 == 0 {
    return false;
  }
  let max_num = (num as f64).sqrt() as u64;
  for i in (3..max_num + 1).step_by(2) {
    if num % i == 0 {
      return false;
    }
  }
  return true;
}

fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(step(g, m, n), exp)
}

#[test]
fn basics_step() {
    testing(2,100,110, Some((101, 103)));
    testing(4,100,110, Some((103, 107)));
    testing(8,30000,100000, Some((30089, 30097)));
    testing(11,30000,100000, None);
    testing(2,10000000,11000000, Some((10000139, 10000141)));
}