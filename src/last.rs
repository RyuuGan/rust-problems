fn main() {
  let amount = last(&["a", "b"]);
  println!("{:?}", amount);
}

fn last<T: Clone>(slice: &[T]) -> T {
  let last = &slice[slice.len() - 1];

  last.clone()
}

#[test]
fn should_work_for_non_empty_list_of_integers() {
  assert_eq!(last(&[1, 2, 3]), 3);
}

#[test]
fn should_work_for_non_empty_list_of_strings() {
  assert_eq!(last(&["a", "b"]), "b");
}
