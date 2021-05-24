use std::fs::File;
use std::io::prelude::*;
use std::io::Seek;

mod items;
mod locations;

fn main() {
    let mut alttpr = File::open("alttpr.sfc").expect("Failed to open rom");

    for loc in &locations::LOCATIONS {
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
