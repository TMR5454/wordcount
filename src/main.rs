use std::env;
use std::fs::File;
use std::io::BufReader;
use wordcount::count;

fn main() {
    let filename = env::args().nth(1).expect("arg 1 is required as FILENAME");

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    let freqs = count(reader, Default::default());
    println!("{:?}", freqs);
}
