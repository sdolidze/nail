use ferris_says::say;
use std::io::{stdout, BufWriter};

pub fn run() {
    let stdout = stdout();
    let message = String::from("Rusty nails!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    say(message.as_bytes(), width, &mut writer).unwrap();
}
