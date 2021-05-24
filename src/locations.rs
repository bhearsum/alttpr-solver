pub struct Location {
    pub rom_addr: u64,
    // Not sure we should be doing this at compilation time, since
    // can have zero, 1, or multiple items
    // It also probably needs to be a separate type, because
    // some locations have multiple sets of items that can unlock them
    // pub requires: [items::Item; 1],
    pub name: &'static str,
}

pub const LOCATIONS: [Location; 222] = [
    Location {
        rom_addr: 0xE9BC,
        name: "Link's House",
    },
    Location {
        rom_addr: 0xE9F2,
        name: "Aginah's Cave",
    },
    Location {
        rom_addr: 0xE9A4,
        name: "Ice Palace - Big Key Chest",
    },
    Location {
        rom_addr: 0xE9D4,
        name: "Ice Palace - Compass Chest",
    },
    Location {
        rom_addr: 0xE9DD,
        name: "Ice Palace - Map Chest",
    },
    Location {
        rom_addr: 0xE9E0,
        name: "Ice Palace - Spike Room",
    },
    Location {
        rom_addr: 0xE995,
        name: "Ice Palace - Freezor Chest",
    },
    Location {
        rom_addr: 0xE9E3,
        name: "Ice Palace - Iced T Room",
    },
    Location {
        rom_addr: 0xE9AA,
        name: "Ice Palace - Big Chest",
    },
    Location {
        rom_addr: 0x180157,
        name: "Ice Palace - Boss",
    },
    Location {
        rom_addr: 0xE977,
        name: "Eastern Palace - Compass Chest",
    },
    Location {
        rom_addr: 0xE97D,
        name: "Eastern Palace - Big Chest",
    },
    Location {
        rom_addr: 0xE9B3,
        name: "Eastern Palace - Cannonball Chest",
    },
    Location {
        rom_addr: 0xE9B9,
        name: "Eastern Palace - Big Key Chest",
    },
    Location {
        rom_addr: 0xE9F5,
        name: "Eastern Palace - Map Chest",
    },
    Location {
        rom_addr: 0x180150,
        name: "Eastern Palace - Boss",
    },
    Location {
        rom_addr: 0x348FF,
        name: "Waterfall Bottle",
    },
    Location {
        rom_addr: 0x3493B,
        name: "Pyramid Bottle",
    },
    Location {
        rom_addr: 0x180161,
        name: "Ganon's Tower - Bob's Torch",
    },
    Location {
        rom_addr: 0xEAB8,
        name: "Ganon's Tower - DMs Room - Top Left",
    },
    Location {
        rom_addr: 0xEABB,
        name: "Ganon's Tower - DMs Room - Top Right",
    },
    Location {
        rom_addr: 0xEABE,
        name: "Ganon's Tower - DMs Room - Bottom Left",
    },
    Location {
        rom_addr: 0xEAC1,
        name: "Ganon's Tower - DMs Room - Bottom Right",
    },
    Location {
        rom_addr: 0xEAC4,
        name: "Ganon's Tower - Randomizer Room - Top Left",
    },
    Location {
        rom_addr: 0xEAC7,
        name: "Ganon's Tower - Randomizer Room - Top Right",
    },
    Location {
        rom_addr: 0xEACA,
        name: "Ganon's Tower - Randomizer Room - Bottom Left",
    },
    Location {
        rom_addr: 0xEACD,
        name: "Ganon's Tower - Randomizer Room - Bottom Right",
    },
    Location {
        rom_addr: 0xEAD0,
        name: "Ganon's Tower - Firesnake Room",
    },
    Location {
        rom_addr: 0xEAD3,
        name: "Ganon's Tower - Map Chest",
    },
    Location {
        rom_addr: 0xEAD6,
        name: "Ganon's Tower - Big Chest",
    },
    Location {
        rom_addr: 0xEAD9,
        name: "Ganon's Tower - Hope Room - Left",
    },
    Location {
        rom_addr: 0xEADC,
        name: "Ganon's Tower - Hope Room - Right",
    },
    Location {
        rom_addr: 0xEADF,
        name: "Ganon's Tower - Bob's Chest",
    },
    Location {
        rom_addr: 0xEAE2,
        name: "Ganon's Tower - Tile Room",
    },
    Location {
        rom_addr: 0xEAE5,
        name: "Ganon's Tower - Compass Room - Top Left",
    },
    Location {
        rom_addr: 0xEAE8,
        name: "Ganon's Tower - Compass Room - Top Right",
    },
    Location {
        rom_addr: 0xEAEB,
        name: "Ganon's Tower - Compass Room - Bottom Left",
    },
    Location {
        rom_addr: 0xEAEE,
        name: "Ganon's Tower - Compass Room - Bottom Right",
    },
    Location {
        rom_addr: 0xEAF1,
        name: "Ganon's Tower - Big Key Chest",
    },
    Location {
        rom_addr: 0xEAF4,
        name: "Ganon's Tower - Big Key Room - Left",
    },
    Location {
        rom_addr: 0xEAF7,
        name: "Ganon's Tower - Big Key Room - Right",
    },
    Location {
        rom_addr: 0xEAFD,
        name: "Ganon's Tower - Mini Helmasaur Room - Left",
    },
    Location {
        rom_addr: 0xEB00,
        name: "Ganon's Tower - Mini Helmasaur Room - Right",
    },
    Location {
        rom_addr: 0xEB03,
        name: "Ganon's Tower - Pre-Moldorm Chest",
    },
    Location {
        rom_addr: 0xEB06,
        name: "Ganon's Tower - Moldorm Chest",
    },
    Location {
        rom_addr: 0xEA9D,
        name: "Swamp Palace - Entrance",
    },
    Location {
        rom_addr: 0xE989,
        name: "Swamp Palace - Big Chest",
    },
    Location {
        rom_addr: 0xEAA6,
        name: "Swamp Palace - Big Key Chest",
    },
    Location {
        rom_addr: 0xE986,
        name: "Swamp Palace - Map Chest",
    },
    Location {
        rom_addr: 0xEAA3,
        name: "Swamp Palace - West Chest",
    },
    Location {
        rom_addr: 0xEAA0,
        name: "Swamp Palace - Compass Chest",
    },
    Location {
        rom_addr: 0xEAA9,
        name: "Swamp Palace - Flooded Room - Left",
    },
    Location {
        rom_addr: 0xEAAC,
        name: "Swamp Palace - Flooded Room - Right",
    },
    Location {
        rom_addr: 0xEAAF,
        name: "Swamp Palace - Waterfall Room",
    },
    Location {
        rom_addr: 0x180154,
        name: "Swamp Palace - Boss",
    },
    Location {
        rom_addr: 0xEA79,
        name: "Sanctuary",
    },
    Location {
        rom_addr: 0xEB5D,
        name: "Sewers - Secret Room - Left",
    },
    Location {
        rom_addr: 0xEB60,
        name: "Sewers - Secret Room - Middle",
    },
    Location {
        rom_addr: 0xEB63,
        name: "Sewers - Secret Room - Right",
    },
    Location {
        rom_addr: 0xE96E,
        name: "Sewers - Dark Cross",
    },
    Location {
        rom_addr: 0xE974,
        name: "Hyrule Castle - Boomerang Chest",
    },
    Location {
        rom_addr: 0xEB0C,
        name: "Hyrule Castle - Map Chest",
    },
    Location {
        rom_addr: 0xEB09,
        name: "Hyrule Castle - Zelda's Cell",
    },
    Location {
        rom_addr: 0x2DF45,
        name: "Link's Uncle",
    },
    Location {
        rom_addr: 0xE971,
        name: "Secret Passage",
    },
    Location {
        rom_addr: 0xEA5B,
        name: "Palace of Darkness - Shooter Room",
    },
    Location {
        rom_addr: 0xEA37,
        name: "Palace of Darkness - Big Key Chest",
    },
    Location {
        rom_addr: 0xEA3A,
        name: "Palace of Darkness - The Arena - Ledge",
    },
    Location {
        rom_addr: 0xEA3D,
        name: "Palace of Darkness - The Arena - Bridge",
    },
    Location {
        rom_addr: 0xEA49,
        name: "Palace of Darkness - Stalfos Basement",
    },
    Location {
        rom_addr: 0xEA52,
        name: "Palace of Darkness - Map Chest",
    },
    Location {
        rom_addr: 0xEA40,
        name: "Palace of Darkness - Big Chest",
    },
    Location {
        rom_addr: 0xEA43,
        name: "Palace of Darkness - Compass Chest",
    },
    Location {
        rom_addr: 0xEA46,
        name: "Palace of Darkness - Harmless Hellway",
    },
    Location {
        rom_addr: 0xEA4C,
        name: "Palace of Darkness - Dark Basement - Left",
    },
    Location {
        rom_addr: 0xEA4F,
        name: "Palace of Darkness - Dark Basement - Right",
    },
    Location {
        rom_addr: 0xEA55,
        name: "Palace of Darkness - Dark Maze - Top",
    },
    Location {
        rom_addr: 0xEA58,
        name: "Palace of Darkness - Dark Maze - Bottom",
    },
    Location {
        rom_addr: 0x180153,
        name: "Palace of Darkness - Boss",
    },
    Location {
        rom_addr: 0xEA16,
        name: "Turtle Rock - Chain Chomps",
    },
    Location {
        rom_addr: 0xEA22,
        name: "Turtle Rock - Compass Chest",
    },
    Location {
        rom_addr: 0xEA1C,
        name: "Turtle Rock - Roller Room - Left",
    },
    Location {
        rom_addr: 0xEA1F,
        name: "Turtle Rock - Roller Room - Right",
    },
    Location {
        rom_addr: 0xEA19,
        name: "Turtle Rock - Big Chest",
    },
    Location {
        rom_addr: 0xEA25,
        name: "Turtle Rock - Big Key Chest",
    },
    Location {
        rom_addr: 0xEA34,
        name: "Turtle Rock - Crystaroller Room",
    },
    Location {
        rom_addr: 0xEA31,
        name: "Turtle Rock - Eye Bridge - Bottom Left",
    },
    Location {
        rom_addr: 0xEA2E,
        name: "Turtle Rock - Eye Bridge - Bottom Right",
    },
    Location {
        rom_addr: 0xEA2B,
        name: "Turtle Rock - Eye Bridge - Top Left",
    },
    Location {
        rom_addr: 0xEA28,
        name: "Turtle Rock - Eye Bridge - Top Right",
    },
    Location {
        rom_addr: 0x180159,
        name: "Turtle Rock - Boss",
    },
    Location {
        rom_addr: 0xE98F,
        name: "Desert Palace - Big Chest",
    },
    Location {
        rom_addr: 0xE9B6,
        name: "Desert Palace - Map Chest",
    },
    Location {
        rom_addr: 0x180160,
        name: "Desert Palace - Torch",
    },
    Location {
        rom_addr: 0xE9C2,
        name: "Desert Palace - Big Key Chest",
    },
    Location {
        rom_addr: 0xE9CB,
        name: "Desert Palace - Compass Chest",
    },
    Location {
        rom_addr: 0x180151,
        name: "Desert Palace - Boss",
    },
    Location {
        rom_addr: 0xEA67,
        name: "Misery Mire - Big Chest",
    },
    Location {
        rom_addr: 0xEA5E,
        name: "Misery Mire - Main Lobby",
    },
    Location {
        rom_addr: 0xEA6D,
        name: "Misery Mire - Big Key Chest",
    },
    Location {
        rom_addr: 0xEA64,
        name: "Misery Mire - Compass Chest",
    },
    Location {
        rom_addr: 0xEA61,
        name: "Misery Mire - Bridge Chest",
    },
    Location {
        rom_addr: 0xEA6A,
        name: "Misery Mire - Map Chest",
    },
    Location {
        rom_addr: 0xE9DA,
        name: "Misery Mire - Spike Chest",
    },
    Location {
        rom_addr: 0x180158,
        name: "Misery Mire - Boss",
    },
    Location {
        rom_addr: 0xE9E6,
        name: "Tower of Hera - Big Key Chest",
    },
    Location {
        rom_addr: 0x180162,
        name: "Tower of Hera - Basement Cage",
    },
    Location {
        rom_addr: 0xE9AD,
        name: "Tower of Hera - Map Chest",
    },
    Location {
        rom_addr: 0xE9FB,
        name: "Tower of Hera - Compass Chest",
    },
    Location {
        rom_addr: 0xE9F8,
        name: "Tower of Hera - Big Chest",
    },
    Location {
        rom_addr: 0x180152,
        name: "Tower of Hera - Boss",
    },
    Location {
        rom_addr: 0x289B0,
        name: "Master Sword Pedestal",
    },
    Location {
        rom_addr: 0xE97A,
        name: "King's Tomb",
    },
    Location {
        rom_addr: 0xE9CE,
        name: "Kakariko Tavern",
    },
    Location {
        rom_addr: 0xE9E9,
        name: "Chicken House",
    },
    Location {
        rom_addr: 0xEA8E,
        name: "Kakariko Well - Top",
    },
    Location {
        rom_addr: 0xEA91,
        name: "Kakariko Well - Left",
    },
    Location {
        rom_addr: 0xEA94,
        name: "Kakariko Well - Middle",
    },
    Location {
        rom_addr: 0xEA97,
        name: "Kakariko Well - Right",
    },
    Location {
        rom_addr: 0xEA9A,
        name: "Kakariko Well - Bottom",
    },
    Location {
        rom_addr: 0xEB0F,
        name: "Blind's Hideout - Top",
    },
    Location {
        rom_addr: 0xEB12,
        name: "Blind's Hideout - Left",
    },
    Location {
        rom_addr: 0xEB15,
        name: "Blind's Hideout - Right",
    },
    Location {
        rom_addr: 0xEB18,
        name: "Blind's Hideout - Far Left",
    },
    Location {
        rom_addr: 0xEB1B,
        name: "Blind's Hideout - Far Right",
    },
    Location {
        rom_addr: 0xEB3F,
        name: "Pegasus Rocks",
    },
    Location {
        rom_addr: 0x2EB18,
        name: "Bottle Merchant",
    },
    Location {
        rom_addr: 0x180015,
        name: "Magic Bat",
    },
    Location {
        rom_addr: 0x339CF,
        name: "Sick Kid",
    },
    Location {
        rom_addr: 0x180000,
        name: "Lost Woods Hideout",
    },
    Location {
        rom_addr: 0x180001,
        name: "Lumberjack Tree",
    },
    Location {
        rom_addr: 0x180004,
        name: "Graveyard Ledge",
    },
    Location {
        rom_addr: 0x180013,
        name: "Mushroom",
    },
    Location {
        rom_addr: 0xEB1E,
        name: "Hype Cave - Top",
    },
    Location {
        rom_addr: 0xEB21,
        name: "Hype Cave - Middle Right",
    },
    Location {
        rom_addr: 0xEB24,
        name: "Hype Cave - Middle Left",
    },
    Location {
        rom_addr: 0xEB27,
        name: "Hype Cave - Bottom",
    },
    Location {
        rom_addr: 0x330C7,
        name: "Stumpy",
    },
    Location {
        rom_addr: 0x180011,
        name: "Hype Cave - NPC",
    },
    Location {
        rom_addr: 0x180148,
        name: "Digging Game",
    },
    Location {
        rom_addr: 0xE9EC,
        name: "Brewery",
    },
    Location {
        rom_addr: 0xE9EF,
        name: "C-Shaped House",
    },
    Location {
        rom_addr: 0xEDA8,
        name: "Chest Game",
    },
    Location {
        rom_addr: 0x180006,
        name: "Hammer Pegs",
    },
    Location {
        rom_addr: 0x180146,
        name: "Bumper Cave",
    },
    // 0x18002A when swords are in pool
    // 0x3355C when they aren't
    Location {
        rom_addr: 0x18002A,
        name: "Blacksmith",
    },
    Location {
        rom_addr: 0x33D68,
        name: "Purple Chest",
    },
    Location {
        rom_addr: 0xEA73,
        name: "Mire Shed - Left",
    },
    Location {
        rom_addr: 0xEA76,
        name: "Mire Shed - Right",
    },
    Location {
        rom_addr: 0xEE185,
        name: "Catfish",
    },
    Location {
        rom_addr: 0x180147,
        name: "Pyramid",
    },
    Location {
        rom_addr: 0x180028,
        name: "Pyramid Fairy - Sword",
    },
    Location {
        rom_addr: 0x34914,
        name: "Pyramid Fairy - Bow",
    },
    Location {
        rom_addr: 0xEA82,
        name: "Sahasrahla's Hut - Left",
    },
    Location {
        rom_addr: 0xEA85,
        name: "Sahasrahla's Hut - Middle",
    },
    Location {
        rom_addr: 0xEA88,
        name: "Sahasrahla's Hut - Right",
    },
    Location {
        rom_addr: 0x2F1FC,
        name: "Sahasrahla",
    },
    Location {
        rom_addr: 0xEE1C3,
        name: "King Zora",
    },
    Location {
        rom_addr: 0x180014,
        name: "Potion Shop",
    },
    Location {
        rom_addr: 0x180149,
        name: "Zora's Ledge",
    },
    Location {
        rom_addr: 0xE9B0,
        name: "Waterfall Fairy - Left",
    },
    Location {
        rom_addr: 0xE9D1,
        name: "Waterfall Fairy - Right",
    },
    Location {
        rom_addr: 0xEA8B,
        name: "Spike Cave",
    },
    Location {
        rom_addr: 0xF69FA,
        name: "Old Man",
    },
    Location {
        rom_addr: 0x180002,
        name: "Spectacle Rock Cave",
    },
    Location {
        rom_addr: 0x180016,
        name: "Ether Tablet",
    },
    Location {
        rom_addr: 0x180140,
        name: "Spectacle Rock",
    },
    Location {
        rom_addr: 0xE9BF,
        name: "Spiral Cave",
    },
    Location {
        rom_addr: 0xE9C5,
        name: "Mimic Cave",
    },
    Location {
        rom_addr: 0xEB2A,
        name: "Paradox Cave Lower - Far Left",
    },
    Location {
        rom_addr: 0xEB2D,
        name: "Paradox Cave Lower - Left",
    },
    Location {
        rom_addr: 0xEB30,
        name: "Paradox Cave Lower - Right",
    },
    Location {
        rom_addr: 0xEB33,
        name: "Paradox Cave Lower - Far Right",
    },
    Location {
        rom_addr: 0xEB36,
        name: "Paradox Cave Lower - Middle",
    },
    Location {
        rom_addr: 0xEB39,
        name: "Paradox Cave Upper - Left",
    },
    Location {
        rom_addr: 0xEB3C,
        name: "Paradox Cave Upper - Right",
    },
    Location {
        rom_addr: 0x180141,
        name: "Floating Island",
    },
    Location {
        rom_addr: 0xEA7C,
        name: "Superbunny Cave - Top",
    },
    Location {
        rom_addr: 0xEA7F,
        name: "Superbunny Cave - Bottom",
    },
    Location {
        rom_addr: 0xEB51,
        name: "Hookshot Cave - Top Right",
    },
    Location {
        rom_addr: 0xEB54,
        name: "Hookshot Cave - Top Left",
    },
    Location {
        rom_addr: 0xEB57,
        name: "Hookshot Cave - Bottom Left",
    },
    Location {
        rom_addr: 0xEB5A,
        name: "Hookshot Cave - Bottom Right",
    },
    Location {
        rom_addr: 0xE98C,
        name: "Floodgate Chest",
    },
    Location {
        rom_addr: 0xE9BC,
        name: "Link's House",
    },
    Location {
        rom_addr: 0xE9F2,
        name: "Aginah's Cave",
    },
    Location {
        rom_addr: 0xEB42,
        name: "Mini Moldorm Cave - Far Left",
    },
    Location {
        rom_addr: 0xEB45,
        name: "Mini Moldorm Cave - Left",
    },
    Location {
        rom_addr: 0xEB48,
        name: "Mini Moldorm Cave - Right",
    },
    Location {
        rom_addr: 0xEB4B,
        name: "Mini Moldorm Cave - Far Right",
    },
    Location {
        rom_addr: 0xEB4E,
        name: "Ice Rod Cave",
    },
    Location {
        rom_addr: 0x33E7D,
        name: "Hobo",
    },
    Location {
        rom_addr: 0x180017,
        name: "Bombos Tablet",
    },
    Location {
        rom_addr: 0x180003,
        name: "Cave 45",
    },
    Location {
        rom_addr: 0x180005,
        name: "Checkerboard Cave",
    },
    Location {
        rom_addr: 0x180010,
        name: "Mini Moldorm Cave - NPC",
    },
    Location {
        rom_addr: 0x180012,
        name: "Library",
    },
    Location {
        rom_addr: 0x180142,
        name: "Maze Race",
    },
    Location {
        rom_addr: 0x180143,
        name: "Desert Ledge",
    },
    Location {
        rom_addr: 0x180144,
        name: "Lake Hylia Island",
    },
    Location {
        rom_addr: 0x180145,
        name: "Sunken Treasure",
    },
    Location {
        rom_addr: 0x18014A,
        name: "Flute Spot",
    },
    Location {
        rom_addr: 0xEAB5,
        name: "Castle Tower - Room 03",
    },
    Location {
        rom_addr: 0xEAB2,
        name: "Castle Tower - Dark Maze",
    },
    Location {
        rom_addr: 0xEA0D,
        name: "Thieves' Town - Attic",
    },
    Location {
        rom_addr: 0xEA04,
        name: "Thieves' Town - Big Key Chest",
    },
    Location {
        rom_addr: 0xEA01,
        name: "Thieves' Town - Map Chest",
    },
    Location {
        rom_addr: 0xEA07,
        name: "Thieves' Town - Compass Chest",
    },
    Location {
        rom_addr: 0xEA0A,
        name: "Thieves' Town - Ambush Chest",
    },
    Location {
        rom_addr: 0xEA10,
        name: "Thieves' Town - Big Chest",
    },
    Location {
        rom_addr: 0xEA13,
        name: "Thieves' Town - Blind's Cell",
    },
    Location {
        rom_addr: 0x180156,
        name: "Thieves' Town - Boss",
    },
    Location {
        rom_addr: 0xE998,
        name: "Skull Woods - Big Chest",
    },
    Location {
        rom_addr: 0xE99E,
        name: "Skull Woods - Big Key Chest",
    },
    Location {
        rom_addr: 0xE992,
        name: "Skull Woods - Compass Chest",
    },
    Location {
        rom_addr: 0xE99B,
        name: "Skull Woods - Map Chest",
    },
    Location {
        rom_addr: 0xE9FE,
        name: "Skull Woods - Bridge Room",
    },
    Location {
        rom_addr: 0xE9A1,
        name: "Skull Woods - Pot Prison",
    },
    Location {
        rom_addr: 0xE9C8,
        name: "Skull Woods - Pinball Room",
    },
    Location {
        rom_addr: 0x180155,
        name: "Skull Woods - Boss",
    },
    Location {
        rom_addr: 0xE980,
        name: "Pyramid Fairy - Left",
    },
    Location {
        rom_addr: 0xE983,
        name: "Pyramid Fairy - Right",
    },
];
