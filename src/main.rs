use ferris_says::say; // Just following the guide
use std::io::{stdout, BufWriter};

fn main() {
    // more knoweledge here
    let stdout = stdout();

    let sms = String::from("Ебало заверни свое пыль, пахта, сметана, закончили!!!!");

    let width = sms.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    say(&sms, width, &mut writer).unwrap();

    //println!("Hello, world!");
}
