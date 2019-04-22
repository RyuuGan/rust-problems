fn main() {
    let s1 = vec![
        "hoqq",
        "bbllkw",
        "oox",
        "ejjuyyy",
        "plmiis",
        "xxxzgpsssa",
        "xxwwkktt",
        "znnnnfqknaz",
        "qqquuhii",
        "dvvvwz",
    ];
    let s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
    let amount = mx_dif_lg(s1, s2);
    println!("{}", amount);
}

fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    if a1.len() == 0 || a2.len() == 0 {
        return -1;
    }

    let a1_max: i32 = a1.iter().map(|&s| s.len() as i32).max().unwrap();
    let a2_max: i32 = a2.iter().map(|&s| s.len() as i32).max().unwrap();

    let a1_min: i32 = a1.iter().map(|&s| s.len() as i32).min().unwrap();
    let a2_min: i32 = a2.iter().map(|&s| s.len() as i32).min().unwrap();

    let max1 = (a1_max - a2_min).abs();
    let max2 = (a1_min - a2_max).abs();

    std::cmp::max(max1, max2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(a1: Vec<&str>, a2: Vec<&str>, exp: i32) -> () {
        println!("a1: {:?};", a1);
        println!("a2: {:?};", a2);
        let ans = mx_dif_lg(a1, a2);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut s1 = vec![
            "hoqq",
            "bbllkw",
            "oox",
            "ejjuyyy",
            "plmiis",
            "xxxzgpsssa",
            "xxwwkktt",
            "znnnnfqknaz",
            "qqquuhii",
            "dvvvwz",
        ];
        let mut s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
        dotest(s1, s2, 13);
        s1 = vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ];
        s2 = vec!["bbbaaayddqbbrrrv"];
        dotest(s1, s2, 10);
    }
}
