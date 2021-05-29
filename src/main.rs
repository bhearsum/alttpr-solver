use std::env;
use std::fs::File;
use std::process;

mod items;
mod locations;
mod rom;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} [rom]", args[0]);
        process::exit(1);
    }

    let mut alttpr = File::open(&args[1]).expect("Failed to open rom");

    let loc_items = rom::find_items_in_rom(&mut alttpr, &locations::LOCATIONS);

    for loc in loc_items.iter() {
        println!("{} contains: {}", loc.location.name, loc.contains.name);
    }
}
