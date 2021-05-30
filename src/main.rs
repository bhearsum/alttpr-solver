use std::collections::HashSet;
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

    let mut current_items: HashSet<&items::Item> = HashSet::new();
    current_items.insert(&items::BOW);
    current_items.insert(&items::FIREROD);
    current_items.insert(&items::POWERGLOVE);

    println!("{} is accessible? {}", locations::EASTERN_PALACE_PRIZE.name, locations::EASTERN_PALACE_PRIZE.is_accessible(&current_items));
    println!("{} is accessible? {}", locations::DESERT_PALACE_BOSS.name, locations::DESERT_PALACE_BOSS.is_accessible(&current_items));
}
