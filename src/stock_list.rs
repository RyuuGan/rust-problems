use std::collections::HashMap;

fn main() {
  let b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
  let c = vec!["A", "B", "C", "D"];
  let amount = stock_list(b, c);
  println!("{}", amount);
}

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.len() == 0 || list_cat.len() == 0 {
      return "".to_string();
    }

    let mut counts:HashMap<char, i32> = HashMap::new();

    for art in list_art {
      let slice: Vec<&str> = art.split(" ").collect();
      let character = slice[0].chars().next().unwrap();
      let current_amount = slice[1].parse::<i32>().unwrap();
      let final_amount = counts.get(&character).unwrap_or(&0) + current_amount;
      counts.insert(character, final_amount);
    }

    let mut result = String::new();

    for idx in 0..list_cat.len() {
      let s = list_cat[idx];
      if s.len() == 0 {
        continue;
      }

      let c = s.chars().next().unwrap();
      let amount = counts.get(&c).unwrap_or(&0);

      let formatted = format!("({} : {})", c, amount);
      result.push_str(&formatted);
      if idx < list_cat.len() - 1 {
        result.push_str(" - ");
      }
    }

    result
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let mut c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");

    }
}
