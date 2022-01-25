use proconio::input;

pub fn run() {
  println!(
    "a1: 値の交換 - exchange of values\n\
    1st: x, 2nd: y"
  );

  input! {
    inx: usize,
    iny: usize,
  }

  let mut x = inx;
  let mut y = iny;

  println!("x = {}, y = {}", &x, &y);

  let tmp = x;
  x = y;
  y = tmp;

  println!("x = {}, y = {}", &x, &y);
}
