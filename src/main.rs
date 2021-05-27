use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Seek;
use std::process;

mod items;
mod locations;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} [rom]", args[0]);
        process::exit(1);
    }

    let mut alttpr = File::open(&args[1]).expect("Failed to open rom");

    for loc in locations::LOCATIONS.iter() {
        alttpr
            .seek(std::io::SeekFrom::Start(loc.rom_addr))
            .expect("Couldn't seek");
        let mut item_value = [0; 1];
        alttpr.read(&mut item_value).expect("Couldn't read");
        println!(
            "{}'s contains: {}",
            loc.name,
            items::get_item(item_value[0]).name
        );
    }
}
