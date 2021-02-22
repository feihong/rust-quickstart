use rand::Rng;
use ferris_says::say;
use std::io::{stdout, BufWriter};

// error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
// static mut RNG: rand::rngs::ThreadRng = rand::thread_rng();

fn gen_hanzi(rng: &mut rand::rngs::ThreadRng) -> char {
  // Doesn't include 0xa000
  rng.gen_range('\u{4e00}'..'\u{a000}')
}

fn main() {
  let mut rng = rand::thread_rng();
  let c = gen_hanzi(&mut rng);
  println!("{}", c);

  let count = 8;
  let mut v: Vec<String> = Vec::with_capacity(count);
  for _i in 0..count {
    let c = gen_hanzi(&mut rng);
    v.push(c.to_string());
  }
  let message = format!("您知道这些汉字吗？{}", v.join(", "));

  let stdout = stdout();
  let message = String::from(message);
  let width = message.chars().count();

  let mut writer = BufWriter::new(stdout.lock());
  say(message.as_bytes(), width, &mut writer).unwrap();
}
