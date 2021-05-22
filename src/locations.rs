use crate::items as items;

pub struct Location {
    pub rom_addr: u32,
    // Not sure we should be doing this at compilation time, since
    // can have zero, 1, or multiple items
    // It also probably needs to be a separate type, because
    // some locations have multiple sets of items that can unlock them
    pub requires: [items::Item; 1],
    pub name: &'static str,
}

pub const LINKS_HOUSE: Location = Location {
    rom_addr: 0xE9BC,
    requires: [items::NOTHING],
    name: "Link's House",
};

pub const AGINAHS_CAVE: Location = Location {
    rom_addr: 0xE9F2,
    requires: [items::NOTHING],
    name: "Aginah's Cave",
};
