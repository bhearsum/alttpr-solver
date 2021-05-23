use std::fs::File;
use std::io::Seek;
use std::io::prelude::*;

mod items;
mod locations;

fn main() {
    let mut alttpr = File::open("alttpr.sfc").expect("Failed to open rom");
    alttpr.seek(std::io::SeekFrom::Start(locations::LINKS_HOUSE.rom_addr)).expect("Couldn't seek");

    let mut item = [0; 1];
    alttpr.read(&mut item).expect("Couldn't read");
    println!("Link's House contains: {:?}", item);
}
