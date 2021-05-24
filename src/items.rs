pub struct Item {
    pub rom_value: u8,
    pub name: &'static str,
}

pub const ITEMS: [Item; 4] = [
    Item {
        rom_value: 0x5A,
        name: "Unknown Item",
    },
    Item {
        rom_value: 0x49,
        name: "Fighter Sword",
    },
    Item {
        rom_value: 0x49,
        name: "Master Sword",
    },
    Item {
        rom_value: 0x41,
        name: "Fifty Rupees",
    },
];

pub fn get_item(rom_value: u8) -> &'static Item {
    for i in &ITEMS {
        if i.rom_value == rom_value {
            return i;
        }
    }

    return &ITEMS[0];
}
