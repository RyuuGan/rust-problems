fn main() {
  let amount = generate_shape(3);
  println!("{:?}", amount);
}

fn generate_shape_mine(n: i32) -> String {
  let str = (0..n).map(|_| "+").collect::<Vec<&str>>().join("");

  (0..n).map(|_| str.clone()).collect::<Vec<String>>().join("\n")
}

fn generate_shape(n: usize) -> String {
    vec!["+".repeat(n); n].join("\n")
}

#[test]
fn sample_test() {
  assert_eq!(generate_shape(3), "+++\n+++\n+++");
}