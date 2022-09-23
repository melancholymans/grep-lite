use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
  let f = File::open("READ.md").unwrap();    // <1>
  let reader = BufReader::new(f);
  for line_ in reader.lines(){
    let line = line_.unwrap(); // <3>
    println!("{} ({} bytes long)", line, line.len());
  }
}