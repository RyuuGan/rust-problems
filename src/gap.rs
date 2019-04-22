fn main() {
  let amount = gap(8, 300, 400);
  println!("{:?}", amount);
}

fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
  let vec: Vec<u64> = (m..(n + 1)).filter(|&n| is_prime(n)).collect();
  if vec.len() < 2 {
    return None;
  }
  for idx in 1..vec.len() {
    if vec[idx] - vec[idx - 1] == g as u64 {
      return Some((vec[idx - 1], vec[idx]));
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
  assert_eq!(gap(g, m, n), exp)
}

#[test]
fn basics_gap() {
  testing(2, 100, 110, Some((101, 103)));
  testing(4, 100, 110, Some((103, 107)));
  testing(6, 100, 110, None);
  testing(8, 300, 400, Some((359, 367)));
}
