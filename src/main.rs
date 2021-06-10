use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::process;

mod items;
mod locations;
mod requirements;
mod rom;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} [rom]", args[0]);
        process::exit(1);
    }

    let mut alttpr = File::open(&args[1]).expect("Failed to open rom");
    let loc_items = rom::find_items_in_rom(&mut alttpr, &locations::LOCATIONS);
    let mut current_items: HashSet<&items::Item> = HashSet::new();
    // we always have bombs, or can get them
    current_items.insert(&items::BOMB);

    // triforce is always behind ganon!
    let triforce_location = locations::LocationItem {
        location: &locations::GANONS_TOWER_GANON_BOSS,
        contains: &items::TRIFORCE,
    };

    for loc_item in loc_items.iter() {
        println!(
            "{} contains: {}",
            loc_item.location.name, loc_item.contains.name
        );
    }

    println!(
        "Dependencies for desert palace boss: {:#?}",
        locations::DESERT_PALACE_BOSS.find_dependencies(&loc_items)
    );

    println!(
        "Triforce requires: {:#?}",
        locations::GANONS_TOWER_GANON_BOSS.requires
    );
    println!(
        "Path to Triforce is: {:#?}",
        locations::GANONS_TOWER_GANON_BOSS.find_path(&loc_items)
    );
}
