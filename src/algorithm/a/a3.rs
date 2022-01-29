use proconio::input;
use rand::{Rng, SeedableRng};
use std::fs;
use std::io::Write;

static RAND_MAX: u32 = 32767;

pub fn run() {
  println!(
    "a3: 暗号 - cryptosystem\n\
    1st: input_file, 2nd: output_file, 3rd: key"
  );

  input! {
    in_in_file: String,
    in_out_file: String,
    in_key: u64
  }

  let raw_str = match fs::read_to_string(in_in_file) {
    Ok(s) => s,
    Err(e) => {
      println!("{}", e);
      std::process::exit(1);
    }
  };
  let mut out_file = match fs::File::create(in_out_file) {
    Ok(f) => f,
    Err(e) => {
      println!("{}", e);
      std::process::exit(1);
    }
  };

  let mut encrypted = "".to_string();
  let mut rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(in_key);
  for c in raw_str.chars() {
    let r = loop {
      let tmpr = rng.gen_range(0..RAND_MAX) / ((RAND_MAX + 1) / 256);
      if tmpr < 256 {
        break tmpr;
      }
    };

    let enc = ((c as u8 ^ r as u8) as char).to_string();
    encrypted.push_str(&enc);
  }

  println!("{}", &encrypted);
  write!(out_file, "{}", encrypted);
}
