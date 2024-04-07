use core::panic;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("assets/lum-marker.json");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Error opening {} because: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't open {} because: {}", display, why),
        Ok(_) => println!("{} contains: \n{}", display, s),
    }
}
