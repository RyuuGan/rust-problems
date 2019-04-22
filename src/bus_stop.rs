fn main() {
    let amount = number1(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]);
    println!("{}", amount);
}

fn number(bus_stops:&[(i32,i32)]) -> i32 {
    let mut result = 0;
    for (a_in, a_out) in bus_stops {
        result = result + a_in - a_out;
    }
    result
}

fn number1(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops.iter().fold(0, |acc, x| acc + x.0 - x.1)
}


#[test]
fn returns_expected() {
  assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
  assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
  assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
}