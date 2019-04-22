fn main() {
  let amount = game(807);
  println!("{:?}", amount);
}

fn game(n: u64) -> Vec<u64> {

  let mut sum: f64 = 0.0;
  let lim = if n % 2 == 1 { n + 1 } else { n };
  for i in 1..lim {
    sum += get_sum(i) * 2.0;
  }
  if n % 2 == 1 {
    sum -= get_sum(n)
  } else {
    sum += get_sum(n)
  }

  println!("{} = {}", sum, sum as u64 as f64);

  if sum == sum as u64 as f64 {
    vec![sum as u64]
  } else {
    vec![(sum * 2.0).round() as u64, 2]
  }
}

fn get_sum(row: u64) -> f64 {
  if row % 2 == 0 {
    return (row / 2) as f64;
  }
  ((row - 1) / 2) as f64 + 0.5
}

fn testing(n: u64, exp: Vec<u64>) -> () {
    assert_eq!(game(n), exp)
}

#[test]
fn basics_game() {

    testing(204, vec![20808]);
    testing(807, vec![651249, 2]);
    testing(5014, vec![12570098]);
    testing(750001, vec![562501500001, 2]);

}
