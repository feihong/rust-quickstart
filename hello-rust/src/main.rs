use rand::Rng;
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let mut rng = rand::thread_rng();
    let c = rng.gen_range('\u{4e00}'..'\u{a000}');
    println!("{}", c);

    let stdout = stdout();
    let message = String::from("你喜欢吃螃蟹吗？");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
