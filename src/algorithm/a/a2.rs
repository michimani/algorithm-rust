use proconio::input;
use std::str::FromStr;

pub fn run() {
  println!(
    "a2: Luhn のアルゴリズム - error detecting code\n\
    1st: credit card number"
  );

  input! {
    innum: String,
  }

  let credit_num = innum;
  let len = credit_num.len();
  let mut total = 0;

  for i in 0..len {
    let idx = len - 1 - i;
    let digit = &credit_num[idx..idx + 1];
    let mut n = usize::from_str(digit).expect("not a number");
    if (len - idx) % 2 == 0 {
      n = times(n);
    }

    total += n;
  }

  if total % 10 == 0 {
    println!("{} is valid number.", &credit_num)
  } else {
    println!("{} is INVALID number.", &credit_num)
  }
}

fn times(n: usize) -> usize {
  let mut t = n * 2;
  if t > 9 {
    t -= 9;
  }

  t
}
