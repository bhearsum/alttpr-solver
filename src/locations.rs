use crate::items::*;

#[derive(Debug)]
pub enum LocationType {
    Unreadable,
    OneAddr(u64),
    TwoAddr([u64; 2]),
}

#[derive(Debug)]
pub struct LocationRequirement {
    item: &'static Item,
    amount: usize,
}

type LocationRequirements = &'static [&'static [LocationRequirement]];

enum ItemRequirement {
    Item,
    AbsentItem,
    ItemIn,
}

pub struct Location {
    pub rom_addrs: LocationType,
    pub name: &'static str,
    // Locations can be locked by any of the follow things:
    // nothing
    // one specific item (ie: back of skull woods is locked by fire rod)
    // one or more progressive items (ie: laser bridge is locked by having 3 shields)
    // something else having been done already (ie: purple chest requires blacksmith to be completed)
    // or any combination of the above, examples include:
    // * hera basement is locked by lamp OR fire rod
    // * king's tomb is locked by boots AND ((moon pearl AND mirror) OR mitts)
    //   * this can be represented by [moon pearl & mirror & boots] | [boots & mitts]
    pub requires: &'static [&'static[&'static ItemRequirement]],
}

impl Location {
    /* pub fn is_accessible(&self, current_items: &HashSet<&Item>) -> bool {
        for req in self.requires {
            let reqset: HashSet<&Item> = HashSet::from_iter(req.iter());
            if current_items.is_superset(&reqset) {
                return true;
            }
        }

        return false;
    } */

    // need to think more about how to represent the possible ways
    // to satisfy the dependencies. especially with progressive items,
    // simply listing all possible variants may end up with a very large
    // list
    pub fn find_dependencies(&self, loc_items: &Vec<LocationItem>) -> Vec<Vec<&Location>> {
        let mut depset: Vec<Vec<&Location>> = Vec::new();

        for reqset in self.requires {
            if !reqset.is_empty() {
                let mut deps: Vec<&Location> = Vec::new();

                for req in reqset.iter() {
                    let locs = get_locations_by_item(loc_items, req.item);
                    // always choose the first spot found for a progressive item - obviously
                    // doesn't work properly
                    for i in 0..req.amount {
                        match locs.get(i) {
                            Some(loc) => deps.push(loc),
                            None => println!("Couldn't get enough {}", req.item.name),
                        }
                    }
                }

                depset.push(deps);
            }
        }

        return depset;
    }

    // finds any possible path to `self`
    pub fn find_path(&self, loc_items: &Vec<LocationItem>) -> Vec<&Location> {
        let mut path: Vec<&Location> = Vec::new();

        if self.requires.len() == 0 {
            return path;
        }

        for reqs in self.requires {
            let locs: Vec<&Location> = get_locations_by_item(loc_items, &reqs[0].item);
            for i in 0..reqs[0].amount {
                match locs.get(i) {
                    Some(loc) => {
                        // doing the path finding for dependencies here
                        // probably means the path is broken whenever we
                        // have more than one dependency, because we should
                        // be adding the path to _all_ dependencies prior
                        // to adding any of our own. i think?
                        path.extend(loc.find_path(loc_items));
                        path.push(loc);
                    }
                    None => println!("Couldn't get enough {}", reqs[0].item.name),
                }
            }
        }

        return path;
    }
}

pub struct LocationItem {
    pub location: &'static Location,
    pub contains: &'static Item,
}

pub fn get_locations_by_item(loc_items: &Vec<LocationItem>, item: &Item) -> Vec<&'static Location> {
    let mut locs: Vec<&Location> = Vec::new();

    for loc in loc_items {
        if loc.contains == item {
            locs.push(loc.location);
        }
    }

    return locs;
}

const ICE_PALACE_BIG_KEY_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9A4),
    requires: &[
        &[
            &PROGRESSIVEGLOVE,
            &PROGRESSIVEGLOVE,
            &HAMMER,
            &CANEOFBYRNA,
            &HOOKSHOT,
        ]
    ],
    name: "Ice Palace - Big Key Chest",
};

const ICE_PALACE_COMPASS_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9D4),
    requires: &[],
    name: "Ice Palace - Compass Chest",
};

const ICE_PALACE_MAP_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9DD),
    requires: &[],
    name: "Ice Palace - Map Chest",
};

const ICE_PALACE_SPIKE_ROOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9E0),
    requires: &[],
    name: "Ice Palace - Spike Room",
};

const ICE_PALACE_FREEZOR_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE995),
    requires: &[],
    name: "Ice Palace - Freezor Chest",
};

const ICE_PALACE_ICED_T_ROOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9E3),
    requires: &[],
    name: "Ice Palace - Iced T Room",
};

const ICE_PALACE_BIG_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9AA),
    requires: &[],
    name: "Ice Palace - Big Chest",
};

const ICE_PALACE_BOSS: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180157),
    requires: &[],
    name: "Ice Palace - Boss",
};

const EASTERN_PALACE_COMPASS_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE977),
    requires: &[], // nothing
    name: "Eastern Palace - Compass Chest",
};

const EASTERN_PALACE_BIG_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE97D),
    requires: &[&[LocationRequirement {
        item: &BIGKEYP1,
        amount: 1,
    }]],
    name: "Eastern Palace - Big Chest",
};

const EASTERN_PALACE_CANNONBALL_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9B3),
    requires: &[], // nothing
    name: "Eastern Palace - Cannonball Chest",
};

const EASTERN_PALACE_BIG_KEY_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9B9),
    requires: &[&[LocationRequirement {
        item: &LAMP,
        amount: 1,
    }]],
    name: "Eastern Palace - Big Key Chest",
};

const EASTERN_PALACE_MAP_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9F5),
    requires: &[], // nothing
    name: "Eastern Palace - Map Chest",
};

const EASTERN_PALACE_BOSS: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180150),
    requires: &[&[LocationRequirement {
        item: &LAMP,
        amount: 1,
    }]],
    name: "Eastern Palace - Boss",
};

const WATERFALL_BOTTLE: Location = Location {
    rom_addrs: LocationType::OneAddr(0x348FF),
    requires: &[],
    name: "Waterfall Bottle",
};

const PYRAMID_BOTTLE: Location = Location {
    rom_addrs: LocationType::OneAddr(0x3493B),
    requires: &[],
    name: "Pyramid Bottle",
};

const GANONS_TOWER_BOBS_TORCH: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180161),
    requires: &[],
    name: "Ganon's Tower - Bob's Torch",
};

const GANONS_TOWER_DMS_ROOM_TOP_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAB8),
    requires: &[],
    name: "Ganon's Tower - DMs Room - Top Left",
};

const GANONS_TOWER_DMS_ROOM_TOP_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEABB),
    requires: &[],
    name: "Ganon's Tower - DMs Room - Top Right",
};

const GANONS_TOWER_DMS_ROOM_BOTTOM_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEABE),
    requires: &[],
    name: "Ganon's Tower - DMs Room - Bottom Left",
};

const GANONS_TOWER_DMS_ROOM_BOTTOM_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAC1),
    requires: &[],
    name: "Ganon's Tower - DMs Room - Bottom Right",
};

const GANONS_TOWER_RANDOMIZER_ROOM_TOP_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAC4),
    requires: &[],
    name: "Ganon's Tower - Randomizer Room - Top Left",
};

const GANONS_TOWER_RANDOMIZER_ROOM_TOP_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAC7),
    requires: &[],
    name: "Ganon's Tower - Randomizer Room - Top Right",
};

const GANONS_TOWER_RANDOMIZER_ROOM_BOTTOM_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEACA),
    requires: &[],
    name: "Ganon's Tower - Randomizer Room - Bottom Left",
};

const GANONS_TOWER_RANDOMIZER_ROOM_BOTTOM_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEACD),
    requires: &[],
    name: "Ganon's Tower - Randomizer Room - Bottom Right",
};

const GANONS_TOWER_FIRESNAKE_ROOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAD0),
    requires: &[],
    name: "Ganon's Tower - Firesnake Room",
};

const GANONS_TOWER_MAP_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAD3),
    requires: &[],
    name: "Ganon's Tower - Map Chest",
};

const GANONS_TOWER_BIG_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAD6),
    requires: &[],
    name: "Ganon's Tower - Big Chest",
};

const GANONS_TOWER_HOPE_ROOM_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAD9),
    requires: &[],
    name: "Ganon's Tower - Hope Room - Left",
};

const GANONS_TOWER_HOPE_ROOM_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEADC),
    requires: &[],
    name: "Ganon's Tower - Hope Room - Right",
};

const GANONS_TOWER_BOBS_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEADF),
    requires: &[],
    name: "Ganon's Tower - Bob's Chest",
};

const GANONS_TOWER_TILE_ROOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAE2),
    requires: &[],
    name: "Ganon's Tower - Tile Room",
};

const GANONS_TOWER_COMPASS_ROOM_TOP_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAE5),
    requires: &[],
    name: "Ganon's Tower - Compass Room - Top Left",
};

const GANONS_TOWER_COMPASS_ROOM_TOP_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAE8),
    requires: &[],
    name: "Ganon's Tower - Compass Room - Top Right",
};

const GANONS_TOWER_COMPASS_ROOM_BOTTOM_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAEB),
    requires: &[],
    name: "Ganon's Tower - Compass Room - Bottom Left",
};

const GANONS_TOWER_COMPASS_ROOM_BOTTOM_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAEE),
    requires: &[],
    name: "Ganon's Tower - Compass Room - Bottom Right",
};

const GANONS_TOWER_BIG_KEY_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAF1),
    requires: &[],
    name: "Ganon's Tower - Big Key Chest",
};

const GANONS_TOWER_BIG_KEY_ROOM_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAF4),
    requires: &[],
    name: "Ganon's Tower - Big Key Room - Left",
};

const GANONS_TOWER_BIG_KEY_ROOM_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAF7),
    requires: &[],
    name: "Ganon's Tower - Big Key Room - Right",
};

const GANONS_TOWER_MINI_HELMASAUR_ROOM_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAFD),
    requires: &[],
    name: "Ganon's Tower - Mini Helmasaur Room - Left",
};

const GANONS_TOWER_MINI_HELMASAUR_ROOM_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB00),
    requires: &[],
    name: "Ganon's Tower - Mini Helmasaur Room - Right",
};

const GANONS_TOWER_PRE_MOLDORM_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB03),
    requires: &[],
    name: "Ganon's Tower - Pre-Moldorm Chest",
};

const GANONS_TOWER_MOLDORM_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB06),
    requires: &[&[LocationRequirement {
        // in non-keysanity, big key's all share the same item value
        // so this usually ends up being not GT's big key
        item: &BIGKEY,
        amount: 1,
    }]],
    name: "Ganon's Tower - Moldorm Chest",
};

pub const GANONS_TOWER_GANON_BOSS: Location = Location {
    rom_addrs: LocationType::Unreadable,
    requires: &[&[LocationRequirement {
        item: &PROGRESSIVESWORD,
        amount: 2,
    }]],
    name: "Ganon's Tower - Ganon",
};

const SWAMP_PALACE_ENTRANCE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA9D),
    requires: &[],
    name: "Swamp Palace - Entrance",
};

const SWAMP_PALACE_BIG_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE989),
    requires: &[],
    name: "Swamp Palace - Big Chest",
};

const SWAMP_PALACE_BIG_KEY_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAA6),
    requires: &[],
    name: "Swamp Palace - Big Key Chest",
};

const SWAMP_PALACE_MAP_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE986),
    requires: &[],
    name: "Swamp Palace - Map Chest",
};

const SWAMP_PALACE_WEST_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAA3),
    requires: &[],
    name: "Swamp Palace - West Chest",
};

const SWAMP_PALACE_COMPASS_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAA0),
    requires: &[],
    name: "Swamp Palace - Compass Chest",
};

const SWAMP_PALACE_FLOODED_ROOM_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAA9),
    requires: &[],
    name: "Swamp Palace - Flooded Room - Left",
};

const SWAMP_PALACE_FLOODED_ROOM_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAAC),
    requires: &[],
    name: "Swamp Palace - Flooded Room - Right",
};

const SWAMP_PALACE_WATERFALL_ROOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAAF),
    requires: &[],
    name: "Swamp Palace - Waterfall Room",
};

const SWAMP_PALACE_BOSS: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180154),
    requires: &[],
    name: "Swamp Palace - Boss",
};

const SANCTUARY: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA79),
    requires: &[], // nothing
    name: "Sanctuary",
};

const SEWERS_SECRET_ROOM_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB5D),
    requires: &[&[LocationRequirement {
        item: &BOMB,
        amount: 1,
    }]],
    name: "Sewers - Secret Room - Left",
};

const SEWERS_SECRET_ROOM_MIDDLE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB60),
    requires: &[&[LocationRequirement {
        item: &BOMB,
        amount: 1,
    }]],
    name: "Sewers - Secret Room - Middle",
};

const SEWERS_SECRET_ROOM_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB63),
    requires: &[&[LocationRequirement {
        item: &BOMB,
        amount: 1,
    }]],
    name: "Sewers - Secret Room - Right",
};

const SEWERS_DARK_CROSS: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE96E),
    requires: &[&[LocationRequirement {
        item: &LAMP,
        amount: 1,
    }]],
    name: "Sewers - Dark Cross",
};

const HYRULE_CASTLE_BOOMERANG_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE974),
    requires: &[], // nothing
    name: "Hyrule Castle - Boomerang Chest",
};

const HYRULE_CASTLE_MAP_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB0C),
    requires: &[], // nothing
    name: "Hyrule Castle - Map Chest",
};

const HYRULE_CASTLE_ZELDAS_CELL: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB09),
    requires: &[], // nothing - or maybe a big key ?
    name: "Hyrule Castle - Zelda's Cell",
};

const LINKS_UNCLE: Location = Location {
    rom_addrs: LocationType::OneAddr(0x2DF45),
    requires: &[], // nothing
    name: "Link's Uncle",
};

const SECRET_PASSAGE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE971),
    requires: &[], // nothing
    name: "Secret Passage",
};

const PALACE_OF_DARKNESS_SHOOTER_ROOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA5B),
    requires: &[],
    name: "Palace of Darkness - Shooter Room",
};

const PALACE_OF_DARKNESS_BIG_KEY_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA37),
    requires: &[],
    name: "Palace of Darkness - Big Key Chest",
};

const PALACE_OF_DARKNESS_THE_ARENA_LEDGE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA3A),
    requires: &[],
    name: "Palace of Darkness - The Arena - Ledge",
};

const PALACE_OF_DARKNESS_THE_ARENA_BRIDGE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA3D),
    requires: &[],
    name: "Palace of Darkness - The Arena - Bridge",
};

const PALACE_OF_DARKNESS_STALFOS_BASEMENT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA49),
    requires: &[],
    name: "Palace of Darkness - Stalfos Basement",
};

const PALACE_OF_DARKNESS_MAP_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA52),
    requires: &[],
    name: "Palace of Darkness - Map Chest",
};

const PALACE_OF_DARKNESS_BIG_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA40),
    requires: &[],
    name: "Palace of Darkness - Big Chest",
};

const PALACE_OF_DARKNESS_COMPASS_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA43),
    requires: &[],
    name: "Palace of Darkness - Compass Chest",
};

const PALACE_OF_DARKNESS_HARMLESS_HELLWAY: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA46),
    requires: &[],
    name: "Palace of Darkness - Harmless Hellway",
};

const PALACE_OF_DARKNESS_DARK_BASEMENT_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA4C),
    requires: &[],
    name: "Palace of Darkness - Dark Basement - Left",
};

const PALACE_OF_DARKNESS_DARK_BASEMENT_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA4F),
    requires: &[],
    name: "Palace of Darkness - Dark Basement - Right",
};

const PALACE_OF_DARKNESS_DARK_MAZE_TOP: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA55),
    requires: &[],
    name: "Palace of Darkness - Dark Maze - Top",
};

const PALACE_OF_DARKNESS_DARK_MAZE_BOTTOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA58),
    requires: &[],
    name: "Palace of Darkness - Dark Maze - Bottom",
};

const PALACE_OF_DARKNESS_BOSS: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180153),
    requires: &[],
    name: "Palace of Darkness - Boss",
};

const TURTLE_ROCK_CHAIN_CHOMPS: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA16),
    requires: &[],
    name: "Turtle Rock - Chain Chomps",
};

const TURTLE_ROCK_COMPASS_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA22),
    requires: &[],
    name: "Turtle Rock - Compass Chest",
};

const TURTLE_ROCK_ROLLER_ROOM_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA1C),
    requires: &[],
    name: "Turtle Rock - Roller Room - Left",
};

const TURTLE_ROCK_ROLLER_ROOM_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA1F),
    requires: &[],
    name: "Turtle Rock - Roller Room - Right",
};

const TURTLE_ROCK_BIG_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA19),
    requires: &[],
    name: "Turtle Rock - Big Chest",
};

const TURTLE_ROCK_BIG_KEY_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA25),
    requires: &[],
    name: "Turtle Rock - Big Key Chest",
};

const TURTLE_ROCK_CRYSTAROLLER_ROOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA34),
    requires: &[],
    name: "Turtle Rock - Crystaroller Room",
};

const TURTLE_ROCK_EYE_BRIDGE_BOTTOM_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA31),
    requires: &[],
    name: "Turtle Rock - Eye Bridge - Bottom Left",
};

const TURTLE_ROCK_EYE_BRIDGE_BOTTOM_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA2E),
    requires: &[],
    name: "Turtle Rock - Eye Bridge - Bottom Right",
};

const TURTLE_ROCK_EYE_BRIDGE_TOP_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA2B),
    requires: &[],
    name: "Turtle Rock - Eye Bridge - Top Left",
};

const TURTLE_ROCK_EYE_BRIDGE_TOP_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA28),
    requires: &[],
    name: "Turtle Rock - Eye Bridge - Top Right",
};

const TURTLE_ROCK_BOSS: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180159),
    requires: &[],
    name: "Turtle Rock - Boss",
};

const DESERT_PALACE_BIG_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE98F),
    requires: &[&[LocationRequirement {
        item: &BIGKEYP2,
        amount: 1,
    }]],
    name: "Desert Palace - Big Chest",
};

const DESERT_PALACE_MAP_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9B6),
    requires: &[], // nothing
    name: "Desert Palace - Map Chest",
};

const DESERT_PALACE_TORCH: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180160),
    requires: &[&[LocationRequirement {
        item: &PEGASUSBOOTS,
        amount: 1,
    }]],
    name: "Desert Palace - Torch",
};

const DESERT_PALACE_BIG_KEY_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9C2),
    requires: &[], // nothing
    name: "Desert Palace - Big Key Chest",
};

const DESERT_PALACE_COMPASS_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9CB),
    requires: &[], // nothing
    name: "Desert Palace - Compass Chest",
};

pub const DESERT_PALACE_BOSS: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180151),
    requires: &[
        &[
            LocationRequirement {
                item: &PROGRESSIVEGLOVE,
                amount: 1,
            },
        ],
        &[
            LocationRequirement {
                item: &LAMP,
                amount: 1,
            },
            LocationRequirement {
                item: &FIREROD,
                amount: 1,
            },
        ],
    ],
    name: "Desert Palace - Boss",
};

const MISERY_MIRE_BIG_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA67),
    requires: &[],
    name: "Misery Mire - Big Chest",
};

const MISERY_MIRE_MAIN_LOBBY: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA5E),
    requires: &[],
    name: "Misery Mire - Main Lobby",
};

const MISERY_MIRE_BIG_KEY_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA6D),
    requires: &[],
    name: "Misery Mire - Big Key Chest",
};

const MISERY_MIRE_COMPASS_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA64),
    requires: &[],
    name: "Misery Mire - Compass Chest",
};

const MISERY_MIRE_BRIDGE_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA61),
    requires: &[],
    name: "Misery Mire - Bridge Chest",
};

const MISERY_MIRE_MAP_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA6A),
    requires: &[],
    name: "Misery Mire - Map Chest",
};

const MISERY_MIRE_SPIKE_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9DA),
    requires: &[],
    name: "Misery Mire - Spike Chest",
};

const MISERY_MIRE_BOSS: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180158),
    requires: &[],
    name: "Misery Mire - Boss",
};

const TOWER_OF_HERA_BIG_KEY_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9E6),
    requires: &[],
    name: "Tower of Hera - Big Key Chest",
};

const TOWER_OF_HERA_BASEMENT_CAGE: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180162),
    requires: &[],
    name: "Tower of Hera - Basement Cage",
};

const TOWER_OF_HERA_MAP_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9AD),
    requires: &[],
    name: "Tower of Hera - Map Chest",
};

const TOWER_OF_HERA_COMPASS_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9FB),
    requires: &[],
    name: "Tower of Hera - Compass Chest",
};

const TOWER_OF_HERA_BIG_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9F8),
    requires: &[],
    name: "Tower of Hera - Big Chest",
};

const TOWER_OF_HERA_BOSS: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180152),
    requires: &[],
    name: "Tower of Hera - Boss",
};

const MASTER_SWORD_PEDESTAL: Location = Location {
    rom_addrs: LocationType::OneAddr(0x289B0),
    requires: &[],
    name: "Master Sword Pedestal",
};

const KINGS_TOMB: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE97A),
    requires: &[],
    name: "King's Tomb",
};

const KAKARIKO_TAVERN: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9CE),
    requires: &[],
    name: "Kakariko Tavern",
};

const CHICKEN_HOUSE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9E9),
    requires: &[],
    name: "Chicken House",
};

const KAKARIKO_WELL_TOP: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA8E),
    requires: &[],
    name: "Kakariko Well - Top",
};

const KAKARIKO_WELL_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA91),
    requires: &[],
    name: "Kakariko Well - Left",
};

const KAKARIKO_WELL_MIDDLE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA94),
    requires: &[],
    name: "Kakariko Well - Middle",
};

const KAKARIKO_WELL_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA97),
    requires: &[],
    name: "Kakariko Well - Right",
};

const KAKARIKO_WELL_BOTTOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA9A),
    requires: &[],
    name: "Kakariko Well - Bottom",
};

const BLINDS_HIDEOUT_TOP: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB0F),
    requires: &[],
    name: "Blind's Hideout - Top",
};

const BLINDS_HIDEOUT_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB12),
    requires: &[],
    name: "Blind's Hideout - Left",
};

const BLINDS_HIDEOUT_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB15),
    requires: &[],
    name: "Blind's Hideout - Right",
};

const BLINDS_HIDEOUT_FAR_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB18),
    requires: &[],
    name: "Blind's Hideout - Far Left",
};

const BLINDS_HIDEOUT_FAR_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB1B),
    requires: &[],
    name: "Blind's Hideout - Far Right",
};

const PEGASUS_ROCKS: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB3F),
    requires: &[],
    name: "Pegasus Rocks",
};

const BOTTLE_MERCHANT: Location = Location {
    rom_addrs: LocationType::OneAddr(0x2EB18),
    requires: &[],
    name: "Bottle Merchant",
};

const MAGIC_BAT: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180015),
    requires: &[],
    name: "Magic Bat",
};

const SICK_KID: Location = Location {
    rom_addrs: LocationType::OneAddr(0x339CF),
    requires: &[],
    name: "Sick Kid",
};

const LOST_WOODS_HIDEOUT: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180000),
    requires: &[],
    name: "Lost Woods Hideout",
};

const LUMBERJACK_TREE: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180001),
    requires: &[],
    name: "Lumberjack Tree",
};

const GRAVEYARD_LEDGE: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180004),
    requires: &[],
    name: "Graveyard Ledge",
};

const MUSHROOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180013),
    requires: &[&[LocationRequirement {
        item: &MUSHROOM_ITEM,
        amount: 1,
    }]],
    name: "Mushroom",
};

const HYPE_CAVE_TOP: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB1E),
    requires: &[],
    name: "Hype Cave - Top",
};

const HYPE_CAVE_MIDDLE_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB21),
    requires: &[],
    name: "Hype Cave - Middle Right",
};

const HYPE_CAVE_MIDDLE_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB24),
    requires: &[],
    name: "Hype Cave - Middle Left",
};

const HYPE_CAVE_BOTTOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB27),
    requires: &[],
    name: "Hype Cave - Bottom",
};

const STUMPY: Location = Location {
    rom_addrs: LocationType::OneAddr(0x330C7),
    requires: &[],
    name: "Stumpy",
};

const HYPE_CAVE_NPC: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180011),
    requires: &[],
    name: "Hype Cave - NPC",
};

const DIGGING_GAME: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180148),
    requires: &[],
    name: "Digging Game",
};

const BREWERY: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9EC),
    requires: &[],
    name: "Brewery",
};

const C_SHAPED_HOUSE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9EF),
    requires: &[],
    name: "C-Shaped House",
};

const CHEST_GAME: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEDA8),
    requires: &[&[LocationRequirement {
        item: &MOONPEARL,
        amount: 1,
    }]],
    name: "Chest Game",
};

const HAMMER_PEGS: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180006),
    requires: &[],
    name: "Hammer Pegs",
};

const BUMPER_CAVE: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180146),
    requires: &[],
    name: "Bumper Cave",
};

const BLACKSMITH: Location = Location {
    rom_addrs: LocationType::OneAddr(0x0), // fixme
    requires: &[],
    name: "Blacksmith",
};

const PURPLE_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0x33D68),
    requires: &[],
    name: "Purple Chest",
};

const MIRE_SHED_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA73),
    requires: &[],
    name: "Mire Shed - Left",
};

const MIRE_SHED_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA76),
    requires: &[],
    name: "Mire Shed - Right",
};

const CATFISH: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEE185),
    requires: &[],
    name: "Catfish",
};

const PYRAMID: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180147),
    requires: &[],
    name: "Pyramid",
};

const PYRAMID_FAIRY_SWORD: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180028),
    requires: &[],
    name: "Pyramid Fairy - Sword",
};

const PYRAMID_FAIRY_BOW: Location = Location {
    rom_addrs: LocationType::OneAddr(0x34914),
    requires: &[],
    name: "Pyramid Fairy - Bow",
};

const SAHASRAHLAS_HUT_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA82),
    requires: &[],
    name: "Sahasrahla's Hut - Left",
};

const SAHASRAHLAS_HUT_MIDDLE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA85),
    requires: &[],
    name: "Sahasrahla's Hut - Middle",
};

const SAHASRAHLAS_HUT_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA88),
    requires: &[],
    name: "Sahasrahla's Hut - Right",
};

const SAHASRAHLA: Location = Location {
    rom_addrs: LocationType::OneAddr(0x2F1FC),
    requires: &[],
    name: "Sahasrahla",
};

const KING_ZORA: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEE1C3),
    requires: &[],
    name: "King Zora",
};

const POTION_SHOP: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180014),
    requires: &[],
    name: "Potion Shop",
};

const ZORAS_LEDGE: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180149),
    requires: &[],
    name: "Zora's Ledge",
};

const WATERFALL_FAIRY_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9B0),
    requires: &[],
    name: "Waterfall Fairy - Left",
};

const WATERFALL_FAIRY_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9D1),
    requires: &[],
    name: "Waterfall Fairy - Right",
};

const SPIKE_CAVE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA8B),
    requires: &[],
    name: "Spike Cave",
};

const OLD_MAN: Location = Location {
    rom_addrs: LocationType::OneAddr(0xF69FA),
    requires: &[],
    name: "Old Man",
};

const SPECTACLE_ROCK_CAVE: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180002),
    requires: &[],
    name: "Spectacle Rock Cave",
};

const ETHER_TABLET: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180016),
    requires: &[],
    name: "Ether Tablet",
};

const SPECTACLE_ROCK: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180140),
    requires: &[],
    name: "Spectacle Rock",
};

const SPIRAL_CAVE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9BF),
    requires: &[],
    name: "Spiral Cave",
};

const MIMIC_CAVE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9C5),
    requires: &[],
    name: "Mimic Cave",
};

const PARADOX_CAVE_LOWER_FAR_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB2A),
    requires: &[],
    name: "Paradox Cave Lower - Far Left",
};

const PARADOX_CAVE_LOWER_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB2D),
    requires: &[],
    name: "Paradox Cave Lower - Left",
};

const PARADOX_CAVE_LOWER_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB30),
    requires: &[],
    name: "Paradox Cave Lower - Right",
};

const PARADOX_CAVE_LOWER_FAR_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB33),
    requires: &[],
    name: "Paradox Cave Lower - Far Right",
};

const PARADOX_CAVE_LOWER_MIDDLE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB36),
    requires: &[],
    name: "Paradox Cave Lower - Middle",
};

const PARADOX_CAVE_UPPER_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB39),
    requires: &[],
    name: "Paradox Cave Upper - Left",
};

const PARADOX_CAVE_UPPER_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB3C),
    requires: &[],
    name: "Paradox Cave Upper - Right",
};

const FLOATING_ISLAND: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180141),
    requires: &[],
    name: "Floating Island",
};

const SUPERBUNNY_CAVE_TOP: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA7C),
    requires: &[],
    name: "Superbunny Cave - Top",
};

const SUPERBUNNY_CAVE_BOTTOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA7F),
    requires: &[],
    name: "Superbunny Cave - Bottom",
};

const HOOKSHOT_CAVE_TOP_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB51),
    requires: &[],
    name: "Hookshot Cave - Top Right",
};

const HOOKSHOT_CAVE_TOP_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB54),
    requires: &[],
    name: "Hookshot Cave - Top Left",
};

const HOOKSHOT_CAVE_BOTTOM_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB57),
    requires: &[],
    name: "Hookshot Cave - Bottom Left",
};

const HOOKSHOT_CAVE_BOTTOM_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB5A),
    requires: &[],
    name: "Hookshot Cave - Bottom Right",
};

const FLOODGATE_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE98C),
    requires: &[],
    name: "Floodgate Chest",
};

const LINKS_HOUSE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9BC),
    requires: &[],
    name: "Link's House",
};

const AGINAHS_CAVE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9F2),
    requires: &[],
    name: "Aginah's Cave",
};

const MINI_MOLDORM_CAVE_FAR_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB42),
    requires: &[],
    name: "Mini Moldorm Cave - Far Left",
};

const MINI_MOLDORM_CAVE_LEFT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB45),
    requires: &[],
    name: "Mini Moldorm Cave - Left",
};

const MINI_MOLDORM_CAVE_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB48),
    requires: &[],
    name: "Mini Moldorm Cave - Right",
};

const MINI_MOLDORM_CAVE_FAR_RIGHT: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB4B),
    requires: &[],
    name: "Mini Moldorm Cave - Far Right",
};

const ICE_ROD_CAVE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEB4E),
    requires: &[],
    name: "Ice Rod Cave",
};

const HOBO: Location = Location {
    rom_addrs: LocationType::OneAddr(0x33E7D),
    requires: &[],
    name: "Hobo",
};

const BOMBOS_TABLET: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180017),
    requires: &[],
    name: "Bombos Tablet",
};

const CAVE_45: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180003),
    requires: &[],
    name: "Cave 45",
};

const CHECKERBOARD_CAVE: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180005),
    requires: &[],
    name: "Checkerboard Cave",
};

const MINI_MOLDORM_CAVE_NPC: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180010),
    requires: &[],
    name: "Mini Moldorm Cave - NPC",
};

const LIBRARY: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180012),
    requires: &[],
    name: "Library",
};

const MAZE_RACE: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180142),
    requires: &[],
    name: "Maze Race",
};

const DESERT_LEDGE: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180143),
    requires: &[],
    name: "Desert Ledge",
};

const LAKE_HYLIA_ISLAND: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180144),
    requires: &[],
    name: "Lake Hylia Island",
};

const SUNKEN_TREASURE: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180145),
    requires: &[],
    name: "Sunken Treasure",
};

const FLUTE_SPOT: Location = Location {
    rom_addrs: LocationType::OneAddr(0x18014A),
    requires: &[],
    name: "Flute Spot",
};

const CASTLE_TOWER_ROOM_03: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAB5),
    requires: &[],
    name: "Castle Tower - Room 03",
};

const CASTLE_TOWER_DARK_MAZE: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEAB2),
    requires: &[],
    name: "Castle Tower - Dark Maze",
};

const THIEVES_TOWN_ATTIC: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA0D),
    requires: &[],
    name: "Thieves' Town - Attic",
};

const THIEVES_TOWN_BIG_KEY_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA04),
    requires: &[],
    name: "Thieves' Town - Big Key Chest",
};

const THIEVES_TOWN_MAP_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA01),
    requires: &[],
    name: "Thieves' Town - Map Chest",
};

const THIEVES_TOWN_COMPASS_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA07),
    requires: &[],
    name: "Thieves' Town - Compass Chest",
};

const THIEVES_TOWN_AMBUSH_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA0A),
    requires: &[],
    name: "Thieves' Town - Ambush Chest",
};

const THIEVES_TOWN_BIG_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA10),
    requires: &[],
    name: "Thieves' Town - Big Chest",
};

const THIEVES_TOWN_BLINDS_CELL: Location = Location {
    rom_addrs: LocationType::OneAddr(0xEA13),
    requires: &[],
    name: "Thieves' Town - Blind's Cell",
};

const THIEVES_TOWN_BOSS: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180156),
    requires: &[],
    name: "Thieves' Town - Boss",
};

const SKULL_WOODS_BIG_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE998),
    requires: &[],
    name: "Skull Woods - Big Chest",
};

const SKULL_WOODS_BIG_KEY_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE99E),
    requires: &[],
    name: "Skull Woods - Big Key Chest",
};

const SKULL_WOODS_COMPASS_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE992),
    requires: &[],
    name: "Skull Woods - Compass Chest",
};

const SKULL_WOODS_MAP_CHEST: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE99B),
    requires: &[],
    name: "Skull Woods - Map Chest",
};

const SKULL_WOODS_BRIDGE_ROOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9FE),
    requires: &[],
    name: "Skull Woods - Bridge Room",
};

const SKULL_WOODS_POT_PRISON: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9A1),
    requires: &[],
    name: "Skull Woods - Pot Prison",
};

const SKULL_WOODS_PINBALL_ROOM: Location = Location {
    rom_addrs: LocationType::OneAddr(0xE9C8),
    requires: &[],
    name: "Skull Woods - Pinball Room",
};

const SKULL_WOODS_BOSS: Location = Location {
    rom_addrs: LocationType::OneAddr(0x180155),
    requires: &[],
    name: "Skull Woods - Boss",
};

const EASTERN_PALACE_PRIZE: Location = Location {
    // the randomizer writes 6 (or maybe 7) locations for prizes, but the first one is enough
    // for our purposes, assuming we treat prizes as "special"
    rom_addrs: LocationType::TwoAddr([0x1209D, 0x18007C]),
    requires: &[&[LocationRequirement {
        item: &LAMP,
        amount: 1,
    }]],
    name: "Eastern Palace - Prize",
};

const DESERT_PALACE_PRIZE: Location = Location {
    rom_addrs: LocationType::TwoAddr([0x1209E, 0x180078]),
    requires: &[],
    name: "Desert Palace - Prize",
};

const TOWER_OF_HERA_PRIZE: Location = Location {
    rom_addrs: LocationType::TwoAddr([0x120A5, 0x18007A]),
    requires: &[],
    name: "Tower of Hera - Prize",
};

const TURTLE_ROCK_PRIZE: Location = Location {
    rom_addrs: LocationType::TwoAddr([0x120A7, 0x180079]),
    requires: &[],
    name: "Turtle Rock - Prize",
};

const ICE_PALACE_PRIZE: Location = Location {
    rom_addrs: LocationType::TwoAddr([0x120A4, 0x180073]),
    requires: &[],
    name: "Ice Palace - Prize",
};

const MISERY_MIRE_PRIZE: Location = Location {
    rom_addrs: LocationType::TwoAddr([0x120A2, 0x180075]),
    requires: &[],
    name: "Misery Mire - Prize",
};

const PALACE_OF_DARKNESS_PRIZE: Location = Location {
    rom_addrs: LocationType::TwoAddr([0x120A1, 0x18007D]),
    requires: &[],
    name: "Palace of Darkness - Prize",
};

const SKULL_WOODS_PRIZE: Location = Location {
    rom_addrs: LocationType::TwoAddr([0x120A3, 0x18007B]),
    requires: &[],
    name: "Skull Woods - Prize",
};

const SWAMP_PALACE_PRIZE: Location = Location {
    rom_addrs: LocationType::TwoAddr([0x120A0, 0x180071]),
    requires: &[],
    name: "Swamp Palace - Prize",
};

const THIEVES_TOWN_PRIZE: Location = Location {
    rom_addrs: LocationType::TwoAddr([0x120A6, 0x180077]),
    requires: &[],
    name: "Thieves' Town - Prize",
};

pub type LocationList = [&'static Location; 228];

pub const LOCATIONS: LocationList = [
    &ICE_PALACE_BIG_KEY_CHEST,
    &ICE_PALACE_COMPASS_CHEST,
    &ICE_PALACE_MAP_CHEST,
    &ICE_PALACE_SPIKE_ROOM,
    &ICE_PALACE_FREEZOR_CHEST,
    &ICE_PALACE_ICED_T_ROOM,
    &ICE_PALACE_BIG_CHEST,
    &ICE_PALACE_BOSS,
    &EASTERN_PALACE_COMPASS_CHEST,
    &EASTERN_PALACE_BIG_CHEST,
    &EASTERN_PALACE_CANNONBALL_CHEST,
    &EASTERN_PALACE_BIG_KEY_CHEST,
    &EASTERN_PALACE_MAP_CHEST,
    &EASTERN_PALACE_BOSS,
    &WATERFALL_BOTTLE,
    &PYRAMID_BOTTLE,
    &GANONS_TOWER_BOBS_TORCH,
    &GANONS_TOWER_DMS_ROOM_TOP_LEFT,
    &GANONS_TOWER_DMS_ROOM_TOP_RIGHT,
    &GANONS_TOWER_DMS_ROOM_BOTTOM_LEFT,
    &GANONS_TOWER_DMS_ROOM_BOTTOM_RIGHT,
    &GANONS_TOWER_RANDOMIZER_ROOM_TOP_LEFT,
    &GANONS_TOWER_RANDOMIZER_ROOM_TOP_RIGHT,
    &GANONS_TOWER_RANDOMIZER_ROOM_BOTTOM_LEFT,
    &GANONS_TOWER_RANDOMIZER_ROOM_BOTTOM_RIGHT,
    &GANONS_TOWER_FIRESNAKE_ROOM,
    &GANONS_TOWER_MAP_CHEST,
    &GANONS_TOWER_BIG_CHEST,
    &GANONS_TOWER_HOPE_ROOM_LEFT,
    &GANONS_TOWER_HOPE_ROOM_RIGHT,
    &GANONS_TOWER_BOBS_CHEST,
    &GANONS_TOWER_TILE_ROOM,
    &GANONS_TOWER_COMPASS_ROOM_TOP_LEFT,
    &GANONS_TOWER_COMPASS_ROOM_TOP_RIGHT,
    &GANONS_TOWER_COMPASS_ROOM_BOTTOM_LEFT,
    &GANONS_TOWER_COMPASS_ROOM_BOTTOM_RIGHT,
    &GANONS_TOWER_BIG_KEY_CHEST,
    &GANONS_TOWER_BIG_KEY_ROOM_LEFT,
    &GANONS_TOWER_BIG_KEY_ROOM_RIGHT,
    &GANONS_TOWER_MINI_HELMASAUR_ROOM_LEFT,
    &GANONS_TOWER_MINI_HELMASAUR_ROOM_RIGHT,
    &GANONS_TOWER_PRE_MOLDORM_CHEST,
    &GANONS_TOWER_MOLDORM_CHEST,
    &SWAMP_PALACE_ENTRANCE,
    &SWAMP_PALACE_BIG_CHEST,
    &SWAMP_PALACE_BIG_KEY_CHEST,
    &SWAMP_PALACE_MAP_CHEST,
    &SWAMP_PALACE_WEST_CHEST,
    &SWAMP_PALACE_COMPASS_CHEST,
    &SWAMP_PALACE_FLOODED_ROOM_LEFT,
    &SWAMP_PALACE_FLOODED_ROOM_RIGHT,
    &SWAMP_PALACE_WATERFALL_ROOM,
    &SWAMP_PALACE_BOSS,
    &SANCTUARY,
    &SEWERS_SECRET_ROOM_LEFT,
    &SEWERS_SECRET_ROOM_MIDDLE,
    &SEWERS_SECRET_ROOM_RIGHT,
    &SEWERS_DARK_CROSS,
    &HYRULE_CASTLE_BOOMERANG_CHEST,
    &HYRULE_CASTLE_MAP_CHEST,
    &HYRULE_CASTLE_ZELDAS_CELL,
    &LINKS_UNCLE,
    &SECRET_PASSAGE,
    &PALACE_OF_DARKNESS_SHOOTER_ROOM,
    &PALACE_OF_DARKNESS_BIG_KEY_CHEST,
    &PALACE_OF_DARKNESS_THE_ARENA_LEDGE,
    &PALACE_OF_DARKNESS_THE_ARENA_BRIDGE,
    &PALACE_OF_DARKNESS_STALFOS_BASEMENT,
    &PALACE_OF_DARKNESS_MAP_CHEST,
    &PALACE_OF_DARKNESS_BIG_CHEST,
    &PALACE_OF_DARKNESS_COMPASS_CHEST,
    &PALACE_OF_DARKNESS_HARMLESS_HELLWAY,
    &PALACE_OF_DARKNESS_DARK_BASEMENT_LEFT,
    &PALACE_OF_DARKNESS_DARK_BASEMENT_RIGHT,
    &PALACE_OF_DARKNESS_DARK_MAZE_TOP,
    &PALACE_OF_DARKNESS_DARK_MAZE_BOTTOM,
    &PALACE_OF_DARKNESS_BOSS,
    &TURTLE_ROCK_CHAIN_CHOMPS,
    &TURTLE_ROCK_COMPASS_CHEST,
    &TURTLE_ROCK_ROLLER_ROOM_LEFT,
    &TURTLE_ROCK_ROLLER_ROOM_RIGHT,
    &TURTLE_ROCK_BIG_CHEST,
    &TURTLE_ROCK_BIG_KEY_CHEST,
    &TURTLE_ROCK_CRYSTAROLLER_ROOM,
    &TURTLE_ROCK_EYE_BRIDGE_BOTTOM_LEFT,
    &TURTLE_ROCK_EYE_BRIDGE_BOTTOM_RIGHT,
    &TURTLE_ROCK_EYE_BRIDGE_TOP_LEFT,
    &TURTLE_ROCK_EYE_BRIDGE_TOP_RIGHT,
    &TURTLE_ROCK_BOSS,
    &DESERT_PALACE_BIG_CHEST,
    &DESERT_PALACE_MAP_CHEST,
    &DESERT_PALACE_TORCH,
    &DESERT_PALACE_BIG_KEY_CHEST,
    &DESERT_PALACE_COMPASS_CHEST,
    &DESERT_PALACE_BOSS,
    &MISERY_MIRE_BIG_CHEST,
    &MISERY_MIRE_MAIN_LOBBY,
    &MISERY_MIRE_BIG_KEY_CHEST,
    &MISERY_MIRE_COMPASS_CHEST,
    &MISERY_MIRE_BRIDGE_CHEST,
    &MISERY_MIRE_MAP_CHEST,
    &MISERY_MIRE_SPIKE_CHEST,
    &MISERY_MIRE_BOSS,
    &TOWER_OF_HERA_BIG_KEY_CHEST,
    &TOWER_OF_HERA_BASEMENT_CAGE,
    &TOWER_OF_HERA_MAP_CHEST,
    &TOWER_OF_HERA_COMPASS_CHEST,
    &TOWER_OF_HERA_BIG_CHEST,
    &TOWER_OF_HERA_BOSS,
    &MASTER_SWORD_PEDESTAL,
    &KINGS_TOMB,
    &KAKARIKO_TAVERN,
    &CHICKEN_HOUSE,
    &KAKARIKO_WELL_TOP,
    &KAKARIKO_WELL_LEFT,
    &KAKARIKO_WELL_MIDDLE,
    &KAKARIKO_WELL_RIGHT,
    &KAKARIKO_WELL_BOTTOM,
    &BLINDS_HIDEOUT_TOP,
    &BLINDS_HIDEOUT_LEFT,
    &BLINDS_HIDEOUT_RIGHT,
    &BLINDS_HIDEOUT_FAR_LEFT,
    &BLINDS_HIDEOUT_FAR_RIGHT,
    &PEGASUS_ROCKS,
    &BOTTLE_MERCHANT,
    &MAGIC_BAT,
    &SICK_KID,
    &LOST_WOODS_HIDEOUT,
    &LUMBERJACK_TREE,
    &GRAVEYARD_LEDGE,
    &MUSHROOM,
    &HYPE_CAVE_TOP,
    &HYPE_CAVE_MIDDLE_RIGHT,
    &HYPE_CAVE_MIDDLE_LEFT,
    &HYPE_CAVE_BOTTOM,
    &STUMPY,
    &HYPE_CAVE_NPC,
    &DIGGING_GAME,
    &BREWERY,
    &C_SHAPED_HOUSE,
    &CHEST_GAME,
    &HAMMER_PEGS,
    &BUMPER_CAVE,
    &BLACKSMITH,
    &PURPLE_CHEST,
    &MIRE_SHED_LEFT,
    &MIRE_SHED_RIGHT,
    &CATFISH,
    &PYRAMID,
    &PYRAMID_FAIRY_SWORD,
    &PYRAMID_FAIRY_BOW,
    &SAHASRAHLAS_HUT_LEFT,
    &SAHASRAHLAS_HUT_MIDDLE,
    &SAHASRAHLAS_HUT_RIGHT,
    &SAHASRAHLA,
    &KING_ZORA,
    &POTION_SHOP,
    &ZORAS_LEDGE,
    &WATERFALL_FAIRY_LEFT,
    &WATERFALL_FAIRY_RIGHT,
    &SPIKE_CAVE,
    &OLD_MAN,
    &SPECTACLE_ROCK_CAVE,
    &ETHER_TABLET,
    &SPECTACLE_ROCK,
    &SPIRAL_CAVE,
    &MIMIC_CAVE,
    &PARADOX_CAVE_LOWER_FAR_LEFT,
    &PARADOX_CAVE_LOWER_LEFT,
    &PARADOX_CAVE_LOWER_RIGHT,
    &PARADOX_CAVE_LOWER_FAR_RIGHT,
    &PARADOX_CAVE_LOWER_MIDDLE,
    &PARADOX_CAVE_UPPER_LEFT,
    &PARADOX_CAVE_UPPER_RIGHT,
    &FLOATING_ISLAND,
    &SUPERBUNNY_CAVE_TOP,
    &SUPERBUNNY_CAVE_BOTTOM,
    &HOOKSHOT_CAVE_TOP_RIGHT,
    &HOOKSHOT_CAVE_TOP_LEFT,
    &HOOKSHOT_CAVE_BOTTOM_LEFT,
    &HOOKSHOT_CAVE_BOTTOM_RIGHT,
    &FLOODGATE_CHEST,
    &LINKS_HOUSE,
    &AGINAHS_CAVE,
    &MINI_MOLDORM_CAVE_FAR_LEFT,
    &MINI_MOLDORM_CAVE_LEFT,
    &MINI_MOLDORM_CAVE_RIGHT,
    &MINI_MOLDORM_CAVE_FAR_RIGHT,
    &ICE_ROD_CAVE,
    &HOBO,
    &BOMBOS_TABLET,
    &CAVE_45,
    &CHECKERBOARD_CAVE,
    &MINI_MOLDORM_CAVE_NPC,
    &LIBRARY,
    &MAZE_RACE,
    &DESERT_LEDGE,
    &LAKE_HYLIA_ISLAND,
    &SUNKEN_TREASURE,
    &FLUTE_SPOT,
    &CASTLE_TOWER_ROOM_03,
    &CASTLE_TOWER_DARK_MAZE,
    &THIEVES_TOWN_ATTIC,
    &THIEVES_TOWN_BIG_KEY_CHEST,
    &THIEVES_TOWN_MAP_CHEST,
    &THIEVES_TOWN_COMPASS_CHEST,
    &THIEVES_TOWN_AMBUSH_CHEST,
    &THIEVES_TOWN_BIG_CHEST,
    &THIEVES_TOWN_BLINDS_CELL,
    &THIEVES_TOWN_BOSS,
    &SKULL_WOODS_BIG_CHEST,
    &SKULL_WOODS_BIG_KEY_CHEST,
    &SKULL_WOODS_COMPASS_CHEST,
    &SKULL_WOODS_MAP_CHEST,
    &SKULL_WOODS_BRIDGE_ROOM,
    &SKULL_WOODS_POT_PRISON,
    &SKULL_WOODS_PINBALL_ROOM,
    &SKULL_WOODS_BOSS,
    &EASTERN_PALACE_PRIZE,
    &DESERT_PALACE_PRIZE,
    &TOWER_OF_HERA_PRIZE,
    &TURTLE_ROCK_PRIZE,
    &THIEVES_TOWN_PRIZE,
    &MISERY_MIRE_PRIZE,
    &SWAMP_PALACE_PRIZE,
    &ICE_PALACE_PRIZE,
    &SKULL_WOODS_PRIZE,
    &PALACE_OF_DARKNESS_PRIZE,
];
