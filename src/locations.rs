pub struct Location {
    pub rom_addr: u64,
    // Not sure we should be doing this at compilation time, since
    // can have zero, 1, or multiple items
    // It also probably needs to be a separate type, because
    // some locations have multiple sets of items that can unlock them
    // pub requires: [items::Item; 1],
    pub name: &'static str,
}

pub const LOCATIONS: [Location; 2] = [
    Location {
        rom_addr: 0xE9BC,
        name: "Link's House",
    },
    Location {
        rom_addr: 0xE9F2,
        name: "Aginah's Cave",
    }
];
