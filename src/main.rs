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

    let mut locs: Vec<locations::LocationItem> = Vec::new();

    for loc in locations::LOCATIONS.iter() {
        match loc.rom_addrs {
            locations::LocationType::OneAddr(addr) => {
                alttpr
                    .seek(std::io::SeekFrom::Start(addr))
                    .expect("Couldn't seek");
                let mut item_value = [0; 1];
                alttpr.read(&mut item_value).expect("Couldn't read");
                locs.push(locations::LocationItem {
                    location: &loc,
                    contains: items::get_item(items::ItemType::OneAddr(item_value[0])),
                });
            },
            locations::LocationType::TwoAddr(addrs) => {
                let mut item_values: [[u8; 1]; 2] = [[0]; 2];
                for i in 0..2 {
                    alttpr
                        .seek(std::io::SeekFrom::Start(addrs[i]))
                        .expect("Couldn't seek");
                    alttpr.read(&mut item_values[i]).expect("Couldn't read");
                }
                locs.push(locations::LocationItem {
                    location: &loc,
                    contains: items::get_item(items::ItemType::TwoAddr([item_values[0][0], item_values[1][0]])),
                });
            }
        }
    }

    for loc in locs.iter() {
        println!("{} contains: {}", loc.location.name, loc.contains.name);
    }
}
