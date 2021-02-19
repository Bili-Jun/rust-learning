// fn main() {

// }
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;

    // let a = 1;
    // println!("Hello, world!");

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}