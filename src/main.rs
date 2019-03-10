#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde;

use std::fs;
use yaml_to::{DBYaml, run};

fn main() {
    let filename = "retrosheet-events.yaml";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    match run(contents) {
        Ok(_) => (),
        Err(e) => eprintln!("{}",e)
    }
}
