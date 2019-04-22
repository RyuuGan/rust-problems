use std::collections::HashMap;

fn main() {
  let r = String::from("123 Main Street St. Louisville OH 43071, 432 Main Long Road St. Louisville OH 43071,786 High Street Pollocksville NY 56432,
          54 Holy Grail Street Niagara Town ZP 32908, 3200 Main Rd. Bern AE 56210,1 Gordon St. Atlanta RE 13000,
          10 Pussy Cat Rd. Chicago EX 34342, 10 Gordon St. Atlanta RE 13000, 58 Gordon Road Atlanta RE 13000,
          22 Tokyo Av. Tedmondville SW 43098, 674 Paris bd. Abbeville AA 45521, 10 Surta Alley Goodtown GG 30654,
          45 Holy Grail Al. Niagara Town ZP 32908, 320 Main Al. Bern AE 56210, 14 Gordon Park Atlanta RE 13000,
          100 Pussy Cat Rd. Chicago EX 34342, 2 Gordon St. Atlanta RE 13000, 5 Gordon Road Atlanta RE 13000,
          2200 Tokyo Av. Tedmondville SW 43098, 67 Paris St. Abbeville AA 45521, 11 Surta Avenue Goodtown GG 30654,
          45 Holy Grail Al. Niagara Town ZP 32918, 320 Main Al. Bern AE 56215, 14 Gordon Park Atlanta RE 13200,
          100 Pussy Cat Rd. Chicago EX 34345, 2 Gordon St. Atlanta RE 13222, 5 Gordon Road Atlanta RE 13001,
          2200 Tokyo Av. Tedmondville SW 43198, 67 Paris St. Abbeville AA 45522, 11 Surta Avenue Goodville GG 30655,
          2222 Tokyo Av. Tedmondville SW 43198, 670 Paris St. Abbeville AA 45522, 114 Surta Avenue Goodville GG 30655,
          2 Holy Grail Street Niagara Town ZP 32908, 3 Main Rd. Bern AE 56210, 77 Gordon St. Atlanta RE 13000,
          100 Pussy Cat Rd. Chicago OH 13201");
  let amount = travel(&r, "AA 45522");
  println!("{:?}", amount);
}

fn travel(r: &str, zipcode: &str) -> String {
  let mut map: HashMap<String, Vec<(String, String)>> = HashMap::new();

  r.split(",")
    .collect::<Vec<&str>>()
    .into_iter()
    .for_each(|s| {
      let address_vec: Vec<&str> = s.trim().split(char::is_whitespace).collect();
      let house_number = address_vec[0].to_string();

      let len = address_vec.len();
      let street = address_vec[1..len - 2].join(" ");
      let zip_code = address_vec[len - 2..len].join(" ");

      let current_vec = map.entry(zip_code).or_insert(Vec::new());
      current_vec.push((street, house_number));
    });

  match map.get_mut(zipcode) {
    Some(v) => {
      let mut result = String::from(zipcode.to_string() + ":");

      let mut streets: Vec<&str> = Vec::new();
      let mut houses: Vec<&str> = Vec::new();
      v.into_iter().for_each(|(street, house)| {
        streets.push(street);
        houses.push(house);
      });

      result.push_str(&streets.join(","));
      result.push_str("/");
      result.push_str(&houses.join(","));

      result
    }
    None => zipcode.to_string() + ":/",
  }
}

fn travel1(r: &str, zipcode: &str) -> String {

  let mut dests: Vec<(String, String)> = Vec::new();

  r.split(",")
    .collect::<Vec<&str>>()
    .into_iter()
    .for_each(|s| {
      let address_vec: Vec<&str> = s.trim().split(char::is_whitespace).collect();
      let house_number = address_vec[0].to_string();

      let len = address_vec.len();
      let street = address_vec[1..len - 2].join(" ");
      let zip_code = address_vec[len - 2..len].join(" ");

      if zip_code == zipcode {
        dests.push((street, house_number));
      }
    });

  if dests.len() == 0 {
    return zipcode.to_string() + ":/";
  }

  let mut result = String::from(zipcode.to_string() + ":");

  let mut streets: Vec<String> = Vec::new();
  let mut houses: Vec<String> = Vec::new();
  dests.into_iter().for_each(|(street, house)| {
    streets.push(street);
    houses.push(house);
  });

  result.push_str(&streets.join(","));
  result.push_str("/");
  result.push_str(&houses.join(","));

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  fn ad() -> String {
    return String::from("123 Main Street St. Louisville OH 43071, 432 Main Long Road St. Louisville OH 43071,786 High Street Pollocksville NY 56432,
          54 Holy Grail Street Niagara Town ZP 32908, 3200 Main Rd. Bern AE 56210,1 Gordon St. Atlanta RE 13000,
          10 Pussy Cat Rd. Chicago EX 34342, 10 Gordon St. Atlanta RE 13000, 58 Gordon Road Atlanta RE 13000,
          22 Tokyo Av. Tedmondville SW 43098, 674 Paris bd. Abbeville AA 45521, 10 Surta Alley Goodtown GG 30654,
          45 Holy Grail Al. Niagara Town ZP 32908, 320 Main Al. Bern AE 56210, 14 Gordon Park Atlanta RE 13000,
          100 Pussy Cat Rd. Chicago EX 34342, 2 Gordon St. Atlanta RE 13000, 5 Gordon Road Atlanta RE 13000,
          2200 Tokyo Av. Tedmondville SW 43098, 67 Paris St. Abbeville AA 45521, 11 Surta Avenue Goodtown GG 30654,
          45 Holy Grail Al. Niagara Town ZP 32918, 320 Main Al. Bern AE 56215, 14 Gordon Park Atlanta RE 13200,
          100 Pussy Cat Rd. Chicago EX 34345, 2 Gordon St. Atlanta RE 13222, 5 Gordon Road Atlanta RE 13001,
          2200 Tokyo Av. Tedmondville SW 43198, 67 Paris St. Abbeville AA 45522, 11 Surta Avenue Goodville GG 30655,
          2222 Tokyo Av. Tedmondville SW 43198, 670 Paris St. Abbeville AA 45522, 114 Surta Avenue Goodville GG 30655,
          2 Holy Grail Street Niagara Town ZP 32908, 3 Main Rd. Bern AE 56210, 77 Gordon St. Atlanta RE 13000,
          100 Pussy Cat Rd. Chicago OH 13201");
  }

  fn dotest(r: &str, zipcode: &str, exp: &str) -> () {
    //println!("r:{:?}", r);
    println!(" zipcode:{:?}", zipcode);
    let ans = travel(r, zipcode);
    println!("actual: {:?}", ans);
    println!("expect: {:?}", exp);
    println!("{}", ans == exp);
    assert_eq!(ans, exp);
    println!("{}", "-");
  }

  #[test]
  fn basic_tests() {
    let r = &ad();
    dotest(
      r,
      "AA 45522",
      "AA 45522:Paris St. Abbeville,Paris St. Abbeville/67,670",
    );
    dotest(r, "OH 430", "OH 430:/");
  }
}
