pub struct Item {
    pub rom_value: u32,
    pub name: &'static str,
}

pub const NOTHING: Item = Item {
    rom_value: 0x0,
    name: "nothing",
};

pub const FIGHTER_SWORD: Item = Item {
    rom_value: 0x49,
    name: "Fighter Sword",
};

pub const MASTER_SWORD: Item = Item {
    rom_value: 0x49,
    name: "Master Sword",
};

pub const FIFTY_RUPEES: Item = Item {
    rom_value: 0x41,
    name: "Fifty Rupees",
};
