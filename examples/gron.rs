//! gron as in this [this](https://github.com/FGRibreau/gron)
extern crate flats;
extern crate serde_json;

use std::io::stdin;

fn main() {
  let json = serde_json::from_reader(stdin()).expect("invalid json");
  for (k, v) in flats::flatten_value(json) {
    println!("{} = {}", k, v)
  }
}
