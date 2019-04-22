fn main() {

    let amount = min_value2(vec![1, 3, 1]);
    println!("{:?}", amount);
}

fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();
    let res_vec: Vec<String> = digits.into_iter().map(|d: i32| d.to_string()).collect();
    res_vec.join("").parse::<i32>().unwrap()
}

fn min_value2(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();

    digits.into_iter().fold(0, |acc, d| acc * 10 + d)
}

#[test]
fn basic_test() {
  assert_eq!(min_value(vec![1, 3, 1]), 13);
  assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
  assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}