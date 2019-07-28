use std::f64::consts::PI;
use std::iter;

fn main() {
    let res = iter_pi(0.1);
    println!("{:?}", res);
}

fn rnd10(f: f64) -> f64 { (f * 1e10).round() / 1e10 }

fn iter_pi(epsilon: f64) -> (i32, f64) {
    // your code

    let mut current: i64 = 1;
    let mut sign: i64 = -1;

    let plus2 = iter::repeat_with(|| {
      let tmp = if current == 0 { 1 } else { current };
      current = tmp + 2;
      sign = -sign;
      (sign * tmp) as f64
    });

    let mut val = 0.0;

    for (i, number) in plus2.enumerate() {
      if (PI - val * 4.0).abs() < epsilon {
        return (i as i32, rnd10(val * 4.0));
      }
      val = val + 1.0 / number;
    }

    (0, 0.0)
}

fn testing(epsilon: f64, exp: (i32, f64)) -> () {
    assert_eq!(iter_pi(epsilon), exp)
}

#[test]
fn tests_iter_pi() {
    testing(0.1, (10, 3.0418396189));
    testing(0.01,  (100, 3.1315929036));
    testing(0.001,  (1000, 3.1405926538));
    testing(0.0001,  (10000, 3.1414926536));
    testing(0.00001, (100001, 3.1416026535));
    testing(0.000001,  (1000001, 3.1415936536));
    testing(0.05,  (20, 3.0916238067));
}