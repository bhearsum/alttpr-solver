use crate::items::*;

pub struct Location {
    pub rom_addr: u64,
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
    pub requires: &'static [&'static [Item]],
}

pub struct PrizeLocation {
    pub rom_addrs: [u64; 2],
    pub name: &'static str,
    pub requires: &'static [&'static [Item]],
}

pub const ICE_PALACE_BIG_KEY_CHEST: Location = Location {
    rom_addr: 0xE9A4,
    requires: &[],
    name: "Ice Palace - Big Key Chest",
};

pub const ICE_PALACE_COMPASS_CHEST: Location = Location {
    rom_addr: 0xE9D4,
    requires: &[],
    name: "Ice Palace - Compass Chest",
};

pub const ICE_PALACE_MAP_CHEST: Location = Location {
    rom_addr: 0xE9DD,
    requires: &[],
    name: "Ice Palace - Map Chest",
};

pub const ICE_PALACE_SPIKE_ROOM: Location = Location {
    rom_addr: 0xE9E0,
    requires: &[],
    name: "Ice Palace - Spike Room",
};

pub const ICE_PALACE_FREEZOR_CHEST: Location = Location {
    rom_addr: 0xE995,
    requires: &[],
    name: "Ice Palace - Freezor Chest",
};

pub const ICE_PALACE_ICED_T_ROOM: Location = Location {
    rom_addr: 0xE9E3,
    requires: &[],
    name: "Ice Palace - Iced T Room",
};

pub const ICE_PALACE_BIG_CHEST: Location = Location {
    rom_addr: 0xE9AA,
    requires: &[],
    name: "Ice Palace - Big Chest",
};

pub const ICE_PALACE_BOSS: Location = Location {
    rom_addr: 0x180157,
    requires: &[],
    name: "Ice Palace - Boss",
};

pub const EASTERN_PALACE_COMPASS_CHEST: Location = Location {
    rom_addr: 0xE977,
    requires: &[], // nothing
    name: "Eastern Palace - Compass Chest",
};

pub const EASTERN_PALACE_BIG_CHEST: Location = Location {
    rom_addr: 0xE97D,
    requires: &[&[BIGKEYP1]],
    name: "Eastern Palace - Big Chest",
};

pub const EASTERN_PALACE_CANNONBALL_CHEST: Location = Location {
    rom_addr: 0xE9B3,
    requires: &[], // nothing
    name: "Eastern Palace - Cannonball Chest",
};

pub const EASTERN_PALACE_BIG_KEY_CHEST: Location = Location {
    rom_addr: 0xE9B9,
    requires: &[&[LAMP], &[FIREROD]],
    name: "Eastern Palace - Big Key Chest",
};

pub const EASTERN_PALACE_MAP_CHEST: Location = Location {
    rom_addr: 0xE9F5,
    requires: &[], // nothing
    name: "Eastern Palace - Map Chest",
};

pub const EASTERN_PALACE_BOSS: Location = Location {
    rom_addr: 0x180150,
    requires: &[&[LAMP]],
    name: "Eastern Palace - Boss",
};

pub const WATERFALL_BOTTLE: Location = Location {
    rom_addr: 0x348FF,
    requires: &[],
    name: "Waterfall Bottle",
};

pub const PYRAMID_BOTTLE: Location = Location {
    rom_addr: 0x3493B,
    requires: &[],
    name: "Pyramid Bottle",
};

pub const GANONS_TOWER_BOBS_TORCH: Location = Location {
    rom_addr: 0x180161,
    requires: &[],
    name: "Ganon's Tower - Bob's Torch",
};

pub const GANONS_TOWER_DMS_ROOM_TOP_LEFT: Location = Location {
    rom_addr: 0xEAB8,
    requires: &[],
    name: "Ganon's Tower - DMs Room - Top Left",
};

pub const GANONS_TOWER_DMS_ROOM_TOP_RIGHT: Location = Location {
    rom_addr: 0xEABB,
    requires: &[],
    name: "Ganon's Tower - DMs Room - Top Right",
};

pub const GANONS_TOWER_DMS_ROOM_BOTTOM_LEFT: Location = Location {
    rom_addr: 0xEABE,
    requires: &[],
    name: "Ganon's Tower - DMs Room - Bottom Left",
};

pub const GANONS_TOWER_DMS_ROOM_BOTTOM_RIGHT: Location = Location {
    rom_addr: 0xEAC1,
    requires: &[],
    name: "Ganon's Tower - DMs Room - Bottom Right",
};

pub const GANONS_TOWER_RANDOMIZER_ROOM_TOP_LEFT: Location = Location {
    rom_addr: 0xEAC4,
    requires: &[],
    name: "Ganon's Tower - Randomizer Room - Top Left",
};

pub const GANONS_TOWER_RANDOMIZER_ROOM_TOP_RIGHT: Location = Location {
    rom_addr: 0xEAC7,
    requires: &[],
    name: "Ganon's Tower - Randomizer Room - Top Right",
};

pub const GANONS_TOWER_RANDOMIZER_ROOM_BOTTOM_LEFT: Location = Location {
    rom_addr: 0xEACA,
    requires: &[],
    name: "Ganon's Tower - Randomizer Room - Bottom Left",
};

pub const GANONS_TOWER_RANDOMIZER_ROOM_BOTTOM_RIGHT: Location = Location {
    rom_addr: 0xEACD,
    requires: &[],
    name: "Ganon's Tower - Randomizer Room - Bottom Right",
};

pub const GANONS_TOWER_FIRESNAKE_ROOM: Location = Location {
    rom_addr: 0xEAD0,
    requires: &[],
    name: "Ganon's Tower - Firesnake Room",
};

pub const GANONS_TOWER_MAP_CHEST: Location = Location {
    rom_addr: 0xEAD3,
    requires: &[],
    name: "Ganon's Tower - Map Chest",
};

pub const GANONS_TOWER_BIG_CHEST: Location = Location {
    rom_addr: 0xEAD6,
    requires: &[],
    name: "Ganon's Tower - Big Chest",
};

pub const GANONS_TOWER_HOPE_ROOM_LEFT: Location = Location {
    rom_addr: 0xEAD9,
    requires: &[],
    name: "Ganon's Tower - Hope Room - Left",
};

pub const GANONS_TOWER_HOPE_ROOM_RIGHT: Location = Location {
    rom_addr: 0xEADC,
    requires: &[],
    name: "Ganon's Tower - Hope Room - Right",
};

pub const GANONS_TOWER_BOBS_CHEST: Location = Location {
    rom_addr: 0xEADF,
    requires: &[],
    name: "Ganon's Tower - Bob's Chest",
};

pub const GANONS_TOWER_TILE_ROOM: Location = Location {
    rom_addr: 0xEAE2,
    requires: &[],
    name: "Ganon's Tower - Tile Room",
};

pub const GANONS_TOWER_COMPASS_ROOM_TOP_LEFT: Location = Location {
    rom_addr: 0xEAE5,
    requires: &[],
    name: "Ganon's Tower - Compass Room - Top Left",
};

pub const GANONS_TOWER_COMPASS_ROOM_TOP_RIGHT: Location = Location {
    rom_addr: 0xEAE8,
    requires: &[],
    name: "Ganon's Tower - Compass Room - Top Right",
};

pub const GANONS_TOWER_COMPASS_ROOM_BOTTOM_LEFT: Location = Location {
    rom_addr: 0xEAEB,
    requires: &[],
    name: "Ganon's Tower - Compass Room - Bottom Left",
};

pub const GANONS_TOWER_COMPASS_ROOM_BOTTOM_RIGHT: Location = Location {
    rom_addr: 0xEAEE,
    requires: &[],
    name: "Ganon's Tower - Compass Room - Bottom Right",
};

pub const GANONS_TOWER_BIG_KEY_CHEST: Location = Location {
    rom_addr: 0xEAF1,
    requires: &[],
    name: "Ganon's Tower - Big Key Chest",
};

pub const GANONS_TOWER_BIG_KEY_ROOM_LEFT: Location = Location {
    rom_addr: 0xEAF4,
    requires: &[],
    name: "Ganon's Tower - Big Key Room - Left",
};

pub const GANONS_TOWER_BIG_KEY_ROOM_RIGHT: Location = Location {
    rom_addr: 0xEAF7,
    requires: &[],
    name: "Ganon's Tower - Big Key Room - Right",
};

pub const GANONS_TOWER_MINI_HELMASAUR_ROOM_LEFT: Location = Location {
    rom_addr: 0xEAFD,
    requires: &[],
    name: "Ganon's Tower - Mini Helmasaur Room - Left",
};

pub const GANONS_TOWER_MINI_HELMASAUR_ROOM_RIGHT: Location = Location {
    rom_addr: 0xEB00,
    requires: &[],
    name: "Ganon's Tower - Mini Helmasaur Room - Right",
};

pub const GANONS_TOWER_PRE_MOLDORM_CHEST: Location = Location {
    rom_addr: 0xEB03,
    requires: &[],
    name: "Ganon's Tower - Pre-Moldorm Chest",
};

pub const GANONS_TOWER_MOLDORM_CHEST: Location = Location {
    rom_addr: 0xEB06,
    requires: &[],
    name: "Ganon's Tower - Moldorm Chest",
};

pub const SWAMP_PALACE_ENTRANCE: Location = Location {
    rom_addr: 0xEA9D,
    requires: &[],
    name: "Swamp Palace - Entrance",
};

pub const SWAMP_PALACE_BIG_CHEST: Location = Location {
    rom_addr: 0xE989,
    requires: &[],
    name: "Swamp Palace - Big Chest",
};

pub const SWAMP_PALACE_BIG_KEY_CHEST: Location = Location {
    rom_addr: 0xEAA6,
    requires: &[],
    name: "Swamp Palace - Big Key Chest",
};

pub const SWAMP_PALACE_MAP_CHEST: Location = Location {
    rom_addr: 0xE986,
    requires: &[],
    name: "Swamp Palace - Map Chest",
};

pub const SWAMP_PALACE_WEST_CHEST: Location = Location {
    rom_addr: 0xEAA3,
    requires: &[],
    name: "Swamp Palace - West Chest",
};

pub const SWAMP_PALACE_COMPASS_CHEST: Location = Location {
    rom_addr: 0xEAA0,
    requires: &[],
    name: "Swamp Palace - Compass Chest",
};

pub const SWAMP_PALACE_FLOODED_ROOM_LEFT: Location = Location {
    rom_addr: 0xEAA9,
    requires: &[],
    name: "Swamp Palace - Flooded Room - Left",
};

pub const SWAMP_PALACE_FLOODED_ROOM_RIGHT: Location = Location {
    rom_addr: 0xEAAC,
    requires: &[],
    name: "Swamp Palace - Flooded Room - Right",
};

pub const SWAMP_PALACE_WATERFALL_ROOM: Location = Location {
    rom_addr: 0xEAAF,
    requires: &[],
    name: "Swamp Palace - Waterfall Room",
};

pub const SWAMP_PALACE_BOSS: Location = Location {
    rom_addr: 0x180154,
    requires: &[],
    name: "Swamp Palace - Boss",
};

pub const SANCTUARY: Location = Location {
    rom_addr: 0xEA79,
    requires: &[], // nothing
    name: "Sanctuary",
};

pub const SEWERS_SECRET_ROOM_LEFT: Location = Location {
    rom_addr: 0xEB5D,
    requires: &[&[BOMB]],
    name: "Sewers - Secret Room - Left",
};

pub const SEWERS_SECRET_ROOM_MIDDLE: Location = Location {
    rom_addr: 0xEB60,
    requires: &[&[BOMB]],
    name: "Sewers - Secret Room - Middle",
};

pub const SEWERS_SECRET_ROOM_RIGHT: Location = Location {
    rom_addr: 0xEB63,
    requires: &[&[BOMB]],
    name: "Sewers - Secret Room - Right",
};

pub const SEWERS_DARK_CROSS: Location = Location {
    rom_addr: 0xE96E,
    requires: &[&[LAMP]],
    name: "Sewers - Dark Cross",
};

pub const HYRULE_CASTLE_BOOMERANG_CHEST: Location = Location {
    rom_addr: 0xE974,
    requires: &[], // nothing
    name: "Hyrule Castle - Boomerang Chest",
};

pub const HYRULE_CASTLE_MAP_CHEST: Location = Location {
    rom_addr: 0xEB0C,
    requires: &[], // nothing
    name: "Hyrule Castle - Map Chest",
};

pub const HYRULE_CASTLE_ZELDAS_CELL: Location = Location {
    rom_addr: 0xEB09,
    requires: &[], // nothing - or maybe a big key ?
    name: "Hyrule Castle - Zelda's Cell",
};

pub const LINKS_UNCLE: Location = Location {
    rom_addr: 0x2DF45,
    requires: &[], // nothing
    name: "Link's Uncle",
};

pub const SECRET_PASSAGE: Location = Location {
    rom_addr: 0xE971,
    requires: &[], // nothing
    name: "Secret Passage",
};

pub const PALACE_OF_DARKNESS_SHOOTER_ROOM: Location = Location {
    rom_addr: 0xEA5B,
    requires: &[],
    name: "Palace of Darkness - Shooter Room",
};

pub const PALACE_OF_DARKNESS_BIG_KEY_CHEST: Location = Location {
    rom_addr: 0xEA37,
    requires: &[],
    name: "Palace of Darkness - Big Key Chest",
};

pub const PALACE_OF_DARKNESS_THE_ARENA_LEDGE: Location = Location {
    rom_addr: 0xEA3A,
    requires: &[],
    name: "Palace of Darkness - The Arena - Ledge",
};

pub const PALACE_OF_DARKNESS_THE_ARENA_BRIDGE: Location = Location {
    rom_addr: 0xEA3D,
    requires: &[],
    name: "Palace of Darkness - The Arena - Bridge",
};

pub const PALACE_OF_DARKNESS_STALFOS_BASEMENT: Location = Location {
    rom_addr: 0xEA49,
    requires: &[],
    name: "Palace of Darkness - Stalfos Basement",
};

pub const PALACE_OF_DARKNESS_MAP_CHEST: Location = Location {
    rom_addr: 0xEA52,
    requires: &[],
    name: "Palace of Darkness - Map Chest",
};

pub const PALACE_OF_DARKNESS_BIG_CHEST: Location = Location {
    rom_addr: 0xEA40,
    requires: &[],
    name: "Palace of Darkness - Big Chest",
};

pub const PALACE_OF_DARKNESS_COMPASS_CHEST: Location = Location {
    rom_addr: 0xEA43,
    requires: &[],
    name: "Palace of Darkness - Compass Chest",
};

pub const PALACE_OF_DARKNESS_HARMLESS_HELLWAY: Location = Location {
    rom_addr: 0xEA46,
    requires: &[],
    name: "Palace of Darkness - Harmless Hellway",
};

pub const PALACE_OF_DARKNESS_DARK_BASEMENT_LEFT: Location = Location {
    rom_addr: 0xEA4C,
    requires: &[],
    name: "Palace of Darkness - Dark Basement - Left",
};

pub const PALACE_OF_DARKNESS_DARK_BASEMENT_RIGHT: Location = Location {
    rom_addr: 0xEA4F,
    requires: &[],
    name: "Palace of Darkness - Dark Basement - Right",
};

pub const PALACE_OF_DARKNESS_DARK_MAZE_TOP: Location = Location {
    rom_addr: 0xEA55,
    requires: &[],
    name: "Palace of Darkness - Dark Maze - Top",
};

pub const PALACE_OF_DARKNESS_DARK_MAZE_BOTTOM: Location = Location {
    rom_addr: 0xEA58,
    requires: &[],
    name: "Palace of Darkness - Dark Maze - Bottom",
};

pub const PALACE_OF_DARKNESS_BOSS: Location = Location {
    rom_addr: 0x180153,
    requires: &[],
    name: "Palace of Darkness - Boss",
};

pub const TURTLE_ROCK_CHAIN_CHOMPS: Location = Location {
    rom_addr: 0xEA16,
    requires: &[],
    name: "Turtle Rock - Chain Chomps",
};

pub const TURTLE_ROCK_COMPASS_CHEST: Location = Location {
    rom_addr: 0xEA22,
    requires: &[],
    name: "Turtle Rock - Compass Chest",
};

pub const TURTLE_ROCK_ROLLER_ROOM_LEFT: Location = Location {
    rom_addr: 0xEA1C,
    requires: &[],
    name: "Turtle Rock - Roller Room - Left",
};

pub const TURTLE_ROCK_ROLLER_ROOM_RIGHT: Location = Location {
    rom_addr: 0xEA1F,
    requires: &[],
    name: "Turtle Rock - Roller Room - Right",
};

pub const TURTLE_ROCK_BIG_CHEST: Location = Location {
    rom_addr: 0xEA19,
    requires: &[],
    name: "Turtle Rock - Big Chest",
};

pub const TURTLE_ROCK_BIG_KEY_CHEST: Location = Location {
    rom_addr: 0xEA25,
    requires: &[],
    name: "Turtle Rock - Big Key Chest",
};

pub const TURTLE_ROCK_CRYSTAROLLER_ROOM: Location = Location {
    rom_addr: 0xEA34,
    requires: &[],
    name: "Turtle Rock - Crystaroller Room",
};

pub const TURTLE_ROCK_EYE_BRIDGE_BOTTOM_LEFT: Location = Location {
    rom_addr: 0xEA31,
    requires: &[],
    name: "Turtle Rock - Eye Bridge - Bottom Left",
};

pub const TURTLE_ROCK_EYE_BRIDGE_BOTTOM_RIGHT: Location = Location {
    rom_addr: 0xEA2E,
    requires: &[],
    name: "Turtle Rock - Eye Bridge - Bottom Right",
};

pub const TURTLE_ROCK_EYE_BRIDGE_TOP_LEFT: Location = Location {
    rom_addr: 0xEA2B,
    requires: &[],
    name: "Turtle Rock - Eye Bridge - Top Left",
};

pub const TURTLE_ROCK_EYE_BRIDGE_TOP_RIGHT: Location = Location {
    rom_addr: 0xEA28,
    requires: &[],
    name: "Turtle Rock - Eye Bridge - Top Right",
};

pub const TURTLE_ROCK_BOSS: Location = Location {
    rom_addr: 0x180159,
    requires: &[],
    name: "Turtle Rock - Boss",
};

pub const DESERT_PALACE_BIG_CHEST: Location = Location {
    rom_addr: 0xE98F,
    requires: &[&[BIGKEYP2]],
    name: "Desert Palace - Big Chest",
};

pub const DESERT_PALACE_MAP_CHEST: Location = Location {
    rom_addr: 0xE9B6,
    requires: &[], // nothing
    name: "Desert Palace - Map Chest",
};

pub const DESERT_PALACE_TORCH: Location = Location {
    rom_addr: 0x180160,
    requires: &[&[PEGASUSBOOTS]],
    name: "Desert Palace - Torch",
};

pub const DESERT_PALACE_BIG_KEY_CHEST: Location = Location {
    rom_addr: 0xE9C2,
    requires: &[], // nothing
    name: "Desert Palace - Big Key Chest",
};

pub const DESERT_PALACE_COMPASS_CHEST: Location = Location {
    rom_addr: 0xE9CB,
    requires: &[], // nothing
    name: "Desert Palace - Compass Chest",
};

pub const DESERT_PALACE_BOSS: Location = Location {
    rom_addr: 0x180151,
    requires: &[&[POWERGLOVE, LAMP], &[POWERGLOVE, FIREROD]],
    name: "Desert Palace - Boss",
};

pub const MISERY_MIRE_BIG_CHEST: Location = Location {
    rom_addr: 0xEA67,
    requires: &[],
    name: "Misery Mire - Big Chest",
};

pub const MISERY_MIRE_MAIN_LOBBY: Location = Location {
    rom_addr: 0xEA5E,
    requires: &[],
    name: "Misery Mire - Main Lobby",
};

pub const MISERY_MIRE_BIG_KEY_CHEST: Location = Location {
    rom_addr: 0xEA6D,
    requires: &[],
    name: "Misery Mire - Big Key Chest",
};

pub const MISERY_MIRE_COMPASS_CHEST: Location = Location {
    rom_addr: 0xEA64,
    requires: &[],
    name: "Misery Mire - Compass Chest",
};

pub const MISERY_MIRE_BRIDGE_CHEST: Location = Location {
    rom_addr: 0xEA61,
    requires: &[],
    name: "Misery Mire - Bridge Chest",
};

pub const MISERY_MIRE_MAP_CHEST: Location = Location {
    rom_addr: 0xEA6A,
    requires: &[],
    name: "Misery Mire - Map Chest",
};

pub const MISERY_MIRE_SPIKE_CHEST: Location = Location {
    rom_addr: 0xE9DA,
    requires: &[],
    name: "Misery Mire - Spike Chest",
};

pub const MISERY_MIRE_BOSS: Location = Location {
    rom_addr: 0x180158,
    requires: &[],
    name: "Misery Mire - Boss",
};

pub const TOWER_OF_HERA_BIG_KEY_CHEST: Location = Location {
    rom_addr: 0xE9E6,
    requires: &[],
    name: "Tower of Hera - Big Key Chest",
};

pub const TOWER_OF_HERA_BASEMENT_CAGE: Location = Location {
    rom_addr: 0x180162,
    requires: &[],
    name: "Tower of Hera - Basement Cage",
};

pub const TOWER_OF_HERA_MAP_CHEST: Location = Location {
    rom_addr: 0xE9AD,
    requires: &[],
    name: "Tower of Hera - Map Chest",
};

pub const TOWER_OF_HERA_COMPASS_CHEST: Location = Location {
    rom_addr: 0xE9FB,
    requires: &[],
    name: "Tower of Hera - Compass Chest",
};

pub const TOWER_OF_HERA_BIG_CHEST: Location = Location {
    rom_addr: 0xE9F8,
    requires: &[],
    name: "Tower of Hera - Big Chest",
};

pub const TOWER_OF_HERA_BOSS: Location = Location {
    rom_addr: 0x180152,
    requires: &[],
    name: "Tower of Hera - Boss",
};

pub const MASTER_SWORD_PEDESTAL: Location = Location {
    rom_addr: 0x289B0,
    requires: &[],
    name: "Master Sword Pedestal",
};

pub const KINGS_TOMB: Location = Location {
    rom_addr: 0xE97A,
    requires: &[],
    name: "King's Tomb",
};

pub const KAKARIKO_TAVERN: Location = Location {
    rom_addr: 0xE9CE,
    requires: &[],
    name: "Kakariko Tavern",
};

pub const CHICKEN_HOUSE: Location = Location {
    rom_addr: 0xE9E9,
    requires: &[],
    name: "Chicken House",
};

pub const KAKARIKO_WELL_TOP: Location = Location {
    rom_addr: 0xEA8E,
    requires: &[],
    name: "Kakariko Well - Top",
};

pub const KAKARIKO_WELL_LEFT: Location = Location {
    rom_addr: 0xEA91,
    requires: &[],
    name: "Kakariko Well - Left",
};

pub const KAKARIKO_WELL_MIDDLE: Location = Location {
    rom_addr: 0xEA94,
    requires: &[],
    name: "Kakariko Well - Middle",
};

pub const KAKARIKO_WELL_RIGHT: Location = Location {
    rom_addr: 0xEA97,
    requires: &[],
    name: "Kakariko Well - Right",
};

pub const KAKARIKO_WELL_BOTTOM: Location = Location {
    rom_addr: 0xEA9A,
    requires: &[],
    name: "Kakariko Well - Bottom",
};

pub const BLINDS_HIDEOUT_TOP: Location = Location {
    rom_addr: 0xEB0F,
    requires: &[],
    name: "Blind's Hideout - Top",
};

pub const BLINDS_HIDEOUT_LEFT: Location = Location {
    rom_addr: 0xEB12,
    requires: &[],
    name: "Blind's Hideout - Left",
};

pub const BLINDS_HIDEOUT_RIGHT: Location = Location {
    rom_addr: 0xEB15,
    requires: &[],
    name: "Blind's Hideout - Right",
};

pub const BLINDS_HIDEOUT_FAR_LEFT: Location = Location {
    rom_addr: 0xEB18,
    requires: &[],
    name: "Blind's Hideout - Far Left",
};

pub const BLINDS_HIDEOUT_FAR_RIGHT: Location = Location {
    rom_addr: 0xEB1B,
    requires: &[],
    name: "Blind's Hideout - Far Right",
};

pub const PEGASUS_ROCKS: Location = Location {
    rom_addr: 0xEB3F,
    requires: &[],
    name: "Pegasus Rocks",
};

pub const BOTTLE_MERCHANT: Location = Location {
    rom_addr: 0x2EB18,
    requires: &[],
    name: "Bottle Merchant",
};

pub const MAGIC_BAT: Location = Location {
    rom_addr: 0x180015,
    requires: &[],
    name: "Magic Bat",
};

pub const SICK_KID: Location = Location {
    rom_addr: 0x339CF,
    requires: &[],
    name: "Sick Kid",
};

pub const LOST_WOODS_HIDEOUT: Location = Location {
    rom_addr: 0x180000,
    requires: &[],
    name: "Lost Woods Hideout",
};

pub const LUMBERJACK_TREE: Location = Location {
    rom_addr: 0x180001,
    requires: &[],
    name: "Lumberjack Tree",
};

pub const GRAVEYARD_LEDGE: Location = Location {
    rom_addr: 0x180004,
    requires: &[],
    name: "Graveyard Ledge",
};

pub const MUSHROOM: Location = Location {
    rom_addr: 0x180013,
    requires: &[],
    name: "Mushroom",
};

pub const HYPE_CAVE_TOP: Location = Location {
    rom_addr: 0xEB1E,
    requires: &[],
    name: "Hype Cave - Top",
};

pub const HYPE_CAVE_MIDDLE_RIGHT: Location = Location {
    rom_addr: 0xEB21,
    requires: &[],
    name: "Hype Cave - Middle Right",
};

pub const HYPE_CAVE_MIDDLE_LEFT: Location = Location {
    rom_addr: 0xEB24,
    requires: &[],
    name: "Hype Cave - Middle Left",
};

pub const HYPE_CAVE_BOTTOM: Location = Location {
    rom_addr: 0xEB27,
    requires: &[],
    name: "Hype Cave - Bottom",
};

pub const STUMPY: Location = Location {
    rom_addr: 0x330C7,
    requires: &[],
    name: "Stumpy",
};

pub const HYPE_CAVE_NPC: Location = Location {
    rom_addr: 0x180011,
    requires: &[],
    name: "Hype Cave - NPC",
};

pub const DIGGING_GAME: Location = Location {
    rom_addr: 0x180148,
    requires: &[],
    name: "Digging Game",
};

pub const BREWERY: Location = Location {
    rom_addr: 0xE9EC,
    requires: &[],
    name: "Brewery",
};

pub const C_SHAPED_HOUSE: Location = Location {
    rom_addr: 0xE9EF,
    requires: &[],
    name: "C-Shaped House",
};

pub const CHEST_GAME: Location = Location {
    rom_addr: 0xEDA8,
    requires: &[],
    name: "Chest Game",
};

pub const HAMMER_PEGS: Location = Location {
    rom_addr: 0x180006,
    requires: &[],
    name: "Hammer Pegs",
};

pub const BUMPER_CAVE: Location = Location {
    rom_addr: 0x180146,
    requires: &[],
    name: "Bumper Cave",
};

pub const BLACKSMITH: Location = Location {
    rom_addr: 0x0, // fixme
    requires: &[],
    name: "Blacksmith",
};

pub const PURPLE_CHEST: Location = Location {
    rom_addr: 0x33D68,
    requires: &[],
    name: "Purple Chest",
};

pub const MIRE_SHED_LEFT: Location = Location {
    rom_addr: 0xEA73,
    requires: &[],
    name: "Mire Shed - Left",
};

pub const MIRE_SHED_RIGHT: Location = Location {
    rom_addr: 0xEA76,
    requires: &[],
    name: "Mire Shed - Right",
};

pub const CATFISH: Location = Location {
    rom_addr: 0xEE185,
    requires: &[],
    name: "Catfish",
};

pub const PYRAMID: Location = Location {
    rom_addr: 0x180147,
    requires: &[],
    name: "Pyramid",
};

pub const PYRAMID_FAIRY_SWORD: Location = Location {
    rom_addr: 0x180028,
    requires: &[],
    name: "Pyramid Fairy - Sword",
};

pub const PYRAMID_FAIRY_BOW: Location = Location {
    rom_addr: 0x34914,
    requires: &[],
    name: "Pyramid Fairy - Bow",
};

pub const SAHASRAHLAS_HUT_LEFT: Location = Location {
    rom_addr: 0xEA82,
    requires: &[],
    name: "Sahasrahla's Hut - Left",
};

pub const SAHASRAHLAS_HUT_MIDDLE: Location = Location {
    rom_addr: 0xEA85,
    requires: &[],
    name: "Sahasrahla's Hut - Middle",
};

pub const SAHASRAHLAS_HUT_RIGHT: Location = Location {
    rom_addr: 0xEA88,
    requires: &[],
    name: "Sahasrahla's Hut - Right",
};

pub const SAHASRAHLA: Location = Location {
    rom_addr: 0x2F1FC,
    requires: &[],
    name: "Sahasrahla",
};

pub const KING_ZORA: Location = Location {
    rom_addr: 0xEE1C3,
    requires: &[],
    name: "King Zora",
};

pub const POTION_SHOP: Location = Location {
    rom_addr: 0x180014,
    requires: &[],
    name: "Potion Shop",
};

pub const ZORAS_LEDGE: Location = Location {
    rom_addr: 0x180149,
    requires: &[],
    name: "Zora's Ledge",
};

pub const WATERFALL_FAIRY_LEFT: Location = Location {
    rom_addr: 0xE9B0,
    requires: &[],
    name: "Waterfall Fairy - Left",
};

pub const WATERFALL_FAIRY_RIGHT: Location = Location {
    rom_addr: 0xE9D1,
    requires: &[],
    name: "Waterfall Fairy - Right",
};

pub const SPIKE_CAVE: Location = Location {
    rom_addr: 0xEA8B,
    requires: &[],
    name: "Spike Cave",
};

pub const OLD_MAN: Location = Location {
    rom_addr: 0xF69FA,
    requires: &[],
    name: "Old Man",
};

pub const SPECTACLE_ROCK_CAVE: Location = Location {
    rom_addr: 0x180002,
    requires: &[],
    name: "Spectacle Rock Cave",
};

pub const ETHER_TABLET: Location = Location {
    rom_addr: 0x180016,
    requires: &[],
    name: "Ether Tablet",
};

pub const SPECTACLE_ROCK: Location = Location {
    rom_addr: 0x180140,
    requires: &[],
    name: "Spectacle Rock",
};

pub const SPIRAL_CAVE: Location = Location {
    rom_addr: 0xE9BF,
    requires: &[],
    name: "Spiral Cave",
};

pub const MIMIC_CAVE: Location = Location {
    rom_addr: 0xE9C5,
    requires: &[],
    name: "Mimic Cave",
};

pub const PARADOX_CAVE_LOWER_FAR_LEFT: Location = Location {
    rom_addr: 0xEB2A,
    requires: &[],
    name: "Paradox Cave Lower - Far Left",
};

pub const PARADOX_CAVE_LOWER_LEFT: Location = Location {
    rom_addr: 0xEB2D,
    requires: &[],
    name: "Paradox Cave Lower - Left",
};

pub const PARADOX_CAVE_LOWER_RIGHT: Location = Location {
    rom_addr: 0xEB30,
    requires: &[],
    name: "Paradox Cave Lower - Right",
};

pub const PARADOX_CAVE_LOWER_FAR_RIGHT: Location = Location {
    rom_addr: 0xEB33,
    requires: &[],
    name: "Paradox Cave Lower - Far Right",
};

pub const PARADOX_CAVE_LOWER_MIDDLE: Location = Location {
    rom_addr: 0xEB36,
    requires: &[],
    name: "Paradox Cave Lower - Middle",
};

pub const PARADOX_CAVE_UPPER_LEFT: Location = Location {
    rom_addr: 0xEB39,
    requires: &[],
    name: "Paradox Cave Upper - Left",
};

pub const PARADOX_CAVE_UPPER_RIGHT: Location = Location {
    rom_addr: 0xEB3C,
    requires: &[],
    name: "Paradox Cave Upper - Right",
};

pub const FLOATING_ISLAND: Location = Location {
    rom_addr: 0x180141,
    requires: &[],
    name: "Floating Island",
};

pub const SUPERBUNNY_CAVE_TOP: Location = Location {
    rom_addr: 0xEA7C,
    requires: &[],
    name: "Superbunny Cave - Top",
};

pub const SUPERBUNNY_CAVE_BOTTOM: Location = Location {
    rom_addr: 0xEA7F,
    requires: &[],
    name: "Superbunny Cave - Bottom",
};

pub const HOOKSHOT_CAVE_TOP_RIGHT: Location = Location {
    rom_addr: 0xEB51,
    requires: &[],
    name: "Hookshot Cave - Top Right",
};

pub const HOOKSHOT_CAVE_TOP_LEFT: Location = Location {
    rom_addr: 0xEB54,
    requires: &[],
    name: "Hookshot Cave - Top Left",
};

pub const HOOKSHOT_CAVE_BOTTOM_LEFT: Location = Location {
    rom_addr: 0xEB57,
    requires: &[],
    name: "Hookshot Cave - Bottom Left",
};

pub const HOOKSHOT_CAVE_BOTTOM_RIGHT: Location = Location {
    rom_addr: 0xEB5A,
    requires: &[],
    name: "Hookshot Cave - Bottom Right",
};

pub const FLOODGATE_CHEST: Location = Location {
    rom_addr: 0xE98C,
    requires: &[],
    name: "Floodgate Chest",
};

pub const LINKS_HOUSE: Location = Location {
    rom_addr: 0xE9BC,
    requires: &[],
    name: "Link's House",
};

pub const AGINAHS_CAVE: Location = Location {
    rom_addr: 0xE9F2,
    requires: &[],
    name: "Aginah's Cave",
};

pub const MINI_MOLDORM_CAVE_FAR_LEFT: Location = Location {
    rom_addr: 0xEB42,
    requires: &[],
    name: "Mini Moldorm Cave - Far Left",
};

pub const MINI_MOLDORM_CAVE_LEFT: Location = Location {
    rom_addr: 0xEB45,
    requires: &[],
    name: "Mini Moldorm Cave - Left",
};

pub const MINI_MOLDORM_CAVE_RIGHT: Location = Location {
    rom_addr: 0xEB48,
    requires: &[],
    name: "Mini Moldorm Cave - Right",
};

pub const MINI_MOLDORM_CAVE_FAR_RIGHT: Location = Location {
    rom_addr: 0xEB4B,
    requires: &[],
    name: "Mini Moldorm Cave - Far Right",
};

pub const ICE_ROD_CAVE: Location = Location {
    rom_addr: 0xEB4E,
    requires: &[],
    name: "Ice Rod Cave",
};

pub const HOBO: Location = Location {
    rom_addr: 0x33E7D,
    requires: &[],
    name: "Hobo",
};

pub const BOMBOS_TABLET: Location = Location {
    rom_addr: 0x180017,
    requires: &[],
    name: "Bombos Tablet",
};

pub const CAVE_45: Location = Location {
    rom_addr: 0x180003,
    requires: &[],
    name: "Cave 45",
};

pub const CHECKERBOARD_CAVE: Location = Location {
    rom_addr: 0x180005,
    requires: &[],
    name: "Checkerboard Cave",
};

pub const MINI_MOLDORM_CAVE_NPC: Location = Location {
    rom_addr: 0x180010,
    requires: &[],
    name: "Mini Moldorm Cave - NPC",
};

pub const LIBRARY: Location = Location {
    rom_addr: 0x180012,
    requires: &[],
    name: "Library",
};

pub const MAZE_RACE: Location = Location {
    rom_addr: 0x180142,
    requires: &[],
    name: "Maze Race",
};

pub const DESERT_LEDGE: Location = Location {
    rom_addr: 0x180143,
    requires: &[],
    name: "Desert Ledge",
};

pub const LAKE_HYLIA_ISLAND: Location = Location {
    rom_addr: 0x180144,
    requires: &[],
    name: "Lake Hylia Island",
};

pub const SUNKEN_TREASURE: Location = Location {
    rom_addr: 0x180145,
    requires: &[],
    name: "Sunken Treasure",
};

pub const FLUTE_SPOT: Location = Location {
    rom_addr: 0x18014A,
    requires: &[],
    name: "Flute Spot",
};

pub const CASTLE_TOWER_ROOM_03: Location = Location {
    rom_addr: 0xEAB5,
    requires: &[],
    name: "Castle Tower - Room 03",
};

pub const CASTLE_TOWER_DARK_MAZE: Location = Location {
    rom_addr: 0xEAB2,
    requires: &[],
    name: "Castle Tower - Dark Maze",
};

pub const THIEVES_TOWN_ATTIC: Location = Location {
    rom_addr: 0xEA0D,
    requires: &[],
    name: "Thieves' Town - Attic",
};

pub const THIEVES_TOWN_BIG_KEY_CHEST: Location = Location {
    rom_addr: 0xEA04,
    requires: &[],
    name: "Thieves' Town - Big Key Chest",
};

pub const THIEVES_TOWN_MAP_CHEST: Location = Location {
    rom_addr: 0xEA01,
    requires: &[],
    name: "Thieves' Town - Map Chest",
};

pub const THIEVES_TOWN_COMPASS_CHEST: Location = Location {
    rom_addr: 0xEA07,
    requires: &[],
    name: "Thieves' Town - Compass Chest",
};

pub const THIEVES_TOWN_AMBUSH_CHEST: Location = Location {
    rom_addr: 0xEA0A,
    requires: &[],
    name: "Thieves' Town - Ambush Chest",
};

pub const THIEVES_TOWN_BIG_CHEST: Location = Location {
    rom_addr: 0xEA10,
    requires: &[],
    name: "Thieves' Town - Big Chest",
};

pub const THIEVES_TOWN_BLINDS_CELL: Location = Location {
    rom_addr: 0xEA13,
    requires: &[],
    name: "Thieves' Town - Blind's Cell",
};

pub const THIEVES_TOWN_BOSS: Location = Location {
    rom_addr: 0x180156,
    requires: &[],
    name: "Thieves' Town - Boss",
};

pub const SKULL_WOODS_BIG_CHEST: Location = Location {
    rom_addr: 0xE998,
    requires: &[],
    name: "Skull Woods - Big Chest",
};

pub const SKULL_WOODS_BIG_KEY_CHEST: Location = Location {
    rom_addr: 0xE99E,
    requires: &[],
    name: "Skull Woods - Big Key Chest",
};

pub const SKULL_WOODS_COMPASS_CHEST: Location = Location {
    rom_addr: 0xE992,
    requires: &[],
    name: "Skull Woods - Compass Chest",
};

pub const SKULL_WOODS_MAP_CHEST: Location = Location {
    rom_addr: 0xE99B,
    requires: &[],
    name: "Skull Woods - Map Chest",
};

pub const SKULL_WOODS_BRIDGE_ROOM: Location = Location {
    rom_addr: 0xE9FE,
    requires: &[],
    name: "Skull Woods - Bridge Room",
};

pub const SKULL_WOODS_POT_PRISON: Location = Location {
    rom_addr: 0xE9A1,
    requires: &[],
    name: "Skull Woods - Pot Prison",
};

pub const SKULL_WOODS_PINBALL_ROOM: Location = Location {
    rom_addr: 0xE9C8,
    requires: &[],
    name: "Skull Woods - Pinball Room",
};

pub const SKULL_WOODS_BOSS: Location = Location {
    rom_addr: 0x180155,
    requires: &[],
    name: "Skull Woods - Boss",
};

pub const EASTERN_PALACE_PRIZE: PrizeLocation = PrizeLocation {
    // the randomizer writes 6 (or maybe 7) locations for prizes, but the first one is enough
    // for our purposes, assuming we treat prizes as "special"
    rom_addrs: [0x1209D, 0x18007C],
    requires: &[&[LAMP]],
    name: "Eastern Palace - Prize",
};

pub const DESERT_PALACE_PRIZE: PrizeLocation = PrizeLocation {
    rom_addrs: [0x1209E, 0x180078],
    requires: &[],
    name: "Desert Palace - Prize",
};

pub const TOWER_OF_HERA_PRIZE: PrizeLocation = PrizeLocation {
    rom_addrs: [0x120A5, 0x18007A],
    requires: &[],
    name: "Tower of Hera - Prize",
};

pub const TURTLE_ROCK_PRIZE: PrizeLocation = PrizeLocation {
    rom_addrs: [0x120A7, 0x180079],
    requires: &[],
    name: "Turtle Rock - Prize",
};

pub const ICE_PALACE_PRIZE: PrizeLocation = PrizeLocation {
    rom_addrs: [0x120A4, 0x180073],
    requires: &[],
    name: "Ice Palace - Prize",
};

pub const MISERY_MIRE_PRIZE: PrizeLocation = PrizeLocation {
    rom_addrs: [0x120A2, 0x180075],
    requires: &[],
    name: "Misery Mire - Prize",
};

pub const PALACE_OF_DARKNESS_PRIZE: PrizeLocation = PrizeLocation {
    rom_addrs: [0x120A1, 0x18007D],
    requires: &[],
    name: "Palace of Darkness - Prize",
};

pub const SKULL_WOODS_PRIZE: PrizeLocation = PrizeLocation {
    rom_addrs: [0x120A3, 0x18007B],
    requires: &[],
    name: "Skull Woods - Prize",
};

pub const SWAMP_PALACE_PRIZE: PrizeLocation = PrizeLocation {
    rom_addrs: [0x120A0, 0x180071],
    requires: &[],
    name: "Swamp Palace - Prize",
};

pub const THIEVES_TOWN_PRIZE: PrizeLocation = PrizeLocation {
    rom_addrs: [0x120A6, 0x180077],
    requires: &[],
    name: "Thieves' Town - Prize",
};

pub const LOCATIONS: [&Location; 218] = [
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
];

pub const PRIZE_LOCATIONS: [&PrizeLocation; 10] = [
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
