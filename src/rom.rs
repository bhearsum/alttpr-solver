use crate::items::{get_item, ItemType};
use crate::locations::{LocationItem, LocationList, LocationType};
use std::fs::File;
use std::io::prelude::*;
use std::io::Seek;

pub fn find_items_in_rom(rom: &mut File, locs: &LocationList) -> Vec<LocationItem> {
    let mut loc_items: Vec<LocationItem> = Vec::new();

    for loc in locs.iter() {
        match loc.rom_addrs {
            LocationType::Unreadable => (),
            LocationType::OneAddr(addr) => {
                rom.seek(std::io::SeekFrom::Start(addr))
                    .expect("Couldn't seek");
                let mut item_value = [0; 1];
                rom.read(&mut item_value).expect("Couldn't read");
                loc_items.push(LocationItem {
                    location: &loc,
                    contains: get_item(ItemType::OneAddr(item_value[0])),
                });
            }
            LocationType::TwoAddr(addrs) => {
                let mut item_values: [[u8; 1]; 2] = [[0]; 2];
                for i in 0..2 {
                    rom.seek(std::io::SeekFrom::Start(addrs[i]))
                        .expect("Couldn't seek");
                    rom.read(&mut item_values[i]).expect("Couldn't read");
                }
                loc_items.push(LocationItem {
                    location: &loc,
                    contains: get_item(ItemType::TwoAddr([item_values[0][0], item_values[1][0]])),
                });
            }
        }
    }

    return loc_items;
}
