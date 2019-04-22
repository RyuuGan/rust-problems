fn main() {
  let amount = selfie_and_rot("fijuoo\nCqYVct\nDrPmMJ\nerfpBA\nkWjFUG\nCVUfyL");

  println!("{}", amount);
}

fn rot(s: &str) -> String {
  s.chars().rev().collect::<String>()
}
fn selfie_and_rot(s: &str) -> String {
  // your code

  let len = s.split("\n").next().unwrap().len();

  let selfie = s
    .split("\n")
    .map(|r| r.to_owned() + &".".repeat(len))
    .collect::<Vec<String>>()
    .join("\n");
  let rot = rot(s)
    .split("\n")
    .map(|r| ".".repeat(len) + r)
    .collect::<Vec<String>>()
    .join("\n");

  selfie + "\n" + &rot
}

// first parameter: dots have to be replaced by function of one variable
fn oper(operation: impl Fn(&str) -> String, s: &str) -> String {
  operation(s)
}

fn testing1(s: &str, exp: &str) -> () {
  assert_eq!(oper(rot, s), exp.to_string())
}
fn testing2(s: &str, exp: &str) -> () {
  assert_eq!(oper(selfie_and_rot, s), exp.to_string())
}

#[test]
fn basics_oper() {
  testing1(
    "fijuoo\nCqYVct\nDrPmMJ\nerfpBA\nkWjFUG\nCVUfyL",
    "LyfUVC\nGUFjWk\nABpfre\nJMmPrD\ntcVYqC\nooujif",
  );
  testing1("rkKv\ncofM\nzXkh\nflCB", "BClf\nhkXz\nMfoc\nvKkr");

  testing2(
    "xZBV\njsbS\nJcpN\nfVnP",
    "xZBV....\njsbS....\nJcpN....\nfVnP....\n....PnVf\n....NpcJ\n....Sbsj\n....VBZx",
  );
  testing2(
    "uLcq\nJkuL\nYirX\nnwMB",
    "uLcq....\nJkuL....\nYirX....\nnwMB....\n....BMwn\n....XriY\n....LukJ\n....qcLu",
  );
}
