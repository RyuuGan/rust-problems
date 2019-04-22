fn main() {

    let amount = product_fib(5895);
    println!("{:?}", amount);
}

fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut m0 = 1;
    let mut m1 = 1;

    while m0 * m1 < prod {
        let acc = m1;
        m1 = m1 + m0;
        m0 = acc;
    }

    return (m0, m1, m0 * m1 == prod)
}

#[test]
fn basics_product_fib() {
    dotest(4895, (55, 89, true));
    dotest(5895, (89, 144, false));
}

fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}