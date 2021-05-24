pub struct Item {
    pub rom_value: u8,
    pub name: &'static str,
}

pub const ITEMS: [Item; 161] = [
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
    Item {
        rom_value: 0x5A,
        name: "Nothing",
    },
    Item {
        rom_value: 0x49,
        name: "L1Sword",
    },
    Item {
        rom_value: 0x00,
        name: "L1SwordAndShield",
    },
    Item {
        rom_value: 0x01,
        name: "L2Sword",
    },
    Item {
        rom_value: 0x50,
        name: "MasterSword",
    },
    Item {
        rom_value: 0x02,
        name: "L3Sword",
    },
    Item {
        rom_value: 0x03,
        name: "L4Sword",
    },
    Item {
        rom_value: 0x04,
        name: "BlueShield",
    },
    Item {
        rom_value: 0x05,
        name: "RedShield",
    },
    Item {
        rom_value: 0x06,
        name: "MirrorShield",
    },
    Item {
        rom_value: 0x07,
        name: "FireRod",
    },
    Item {
        rom_value: 0x08,
        name: "IceRod",
    },
    Item {
        rom_value: 0x09,
        name: "Hammer",
    },
    Item {
        rom_value: 0x0A,
        name: "Hookshot",
    },
    Item {
        rom_value: 0x0B,
        name: "Bow",
    },
    Item {
        rom_value: 0x0C,
        name: "Boomerang",
    },
    Item {
        rom_value: 0x0D,
        name: "Powder",
    },
    Item {
        rom_value: 0x0E,
        name: "Bee",
    },
    Item {
        rom_value: 0x12,
        name: "Lamp",
    },
    Item {
        rom_value: 0x13,
        name: "Shovel",
    },
    Item {
        rom_value: 0x14,
        name: "OcarinaInactive",
    },
    Item {
        rom_value: 0x15,
        name: "CaneOfSomaria",
    },
    Item {
        rom_value: 0x16,
        name: "Bottle",
    },
    Item {
        rom_value: 0x17,
        name: "PieceOfHeart",
    },
    Item {
        rom_value: 0x18,
        name: "CaneOfByrna",
    },
    Item {
        rom_value: 0x19,
        name: "Cape",
    },
    Item {
        rom_value: 0x1A,
        name: "MagicMirror",
    },
    Item {
        rom_value: 0x1B,
        name: "PowerGlove",
    },
    Item {
        rom_value: 0x1C,
        name: "TitansMitt",
    },
    Item {
        rom_value: 0x1D,
        name: "BookOfMudora",
    },
    Item {
        rom_value: 0x1E,
        name: "Flippers",
    },
    Item {
        rom_value: 0x1F,
        name: "MoonPearl",
    },
    Item {
        rom_value: 0x21,
        name: "BugCatchingNet",
    },
    Item {
        rom_value: 0x22,
        name: "BlueMail",
    },
    Item {
        rom_value: 0x23,
        name: "RedMail",
    },
    Item {
        rom_value: 0x24,
        name: "Key",
    },
    Item {
        rom_value: 0x25,
        name: "Compass",
    },
    Item {
        rom_value: 0x26,
        name: "HeartContainerNoAnimation",
    },
    Item {
        rom_value: 0x27,
        name: "Bomb",
    },
    Item {
        rom_value: 0x28,
        name: "ThreeBombs",
    },
    Item {
        rom_value: 0x29,
        name: "Mushroom",
    },
    Item {
        rom_value: 0x2A,
        name: "RedBoomerang",
    },
    Item {
        rom_value: 0x2B,
        name: "BottleWithRedPotion",
    },
    Item {
        rom_value: 0x2C,
        name: "BottleWithGreenPotion",
    },
    Item {
        rom_value: 0x2D,
        name: "BottleWithBluePotion",
    },
    Item {
        rom_value: 0x2E,
        name: "RedPotion",
    },
    Item {
        rom_value: 0x2F,
        name: "GreenPotion",
    },
    Item {
        rom_value: 0x30,
        name: "BluePotion",
    },
    Item {
        rom_value: 0x31,
        name: "TenBombs",
    },
    Item {
        rom_value: 0x32,
        name: "BigKey",
    },
    Item {
        rom_value: 0x33,
        name: "Map",
    },
    Item {
        rom_value: 0x34,
        name: "OneRupee",
    },
    Item {
        rom_value: 0x35,
        name: "FiveRupees",
    },
    Item {
        rom_value: 0x36,
        name: "TwentyRupees",
    },
    Item {
        rom_value: 0x3A,
        name: "BowAndArrows",
    },
    Item {
        rom_value: 0x3B,
        name: "BowAndSilverArrows",
    },
    Item {
        rom_value: 0x3C,
        name: "BottleWithBee",
    },
    Item {
        rom_value: 0x3D,
        name: "BottleWithFairy",
    },
    Item {
        rom_value: 0x3E,
        name: "BossHeartContainer",
    },
    Item {
        rom_value: 0x3F,
        name: "HeartContainer",
    },
    Item {
        rom_value: 0x40,
        name: "OneHundredRupees",
    },
    Item {
        rom_value: 0x41,
        name: "FiftyRupees",
    },
    Item {
        rom_value: 0x42,
        name: "Heart",
    },
    Item {
        rom_value: 0x43,
        name: "Arrow",
    },
    Item {
        rom_value: 0x44,
        name: "TenArrows",
    },
    Item {
        rom_value: 0x45,
        name: "SmallMagic",
    },
    Item {
        rom_value: 0x46,
        name: "ThreeHundredRupees",
    },
    Item {
        rom_value: 0x47,
        name: "TwentyRupees2",
    },
    Item {
        rom_value: 0x48,
        name: "BottleWithGoldBee",
    },
    Item {
        rom_value: 0x4A,
        name: "OcarinaActive",
    },
    Item {
        rom_value: 0x4B,
        name: "PegasusBoots",
    },
    Item {
        rom_value: 0x51,
        name: "BombUpgrade5",
    },
    Item {
        rom_value: 0x52,
        name: "BombUpgrade10",
    },
    Item {
        rom_value: 0x4C,
        name: "BombUpgrade50",
    },
    Item {
        rom_value: 0x53,
        name: "ArrowUpgrade5",
    },
    Item {
        rom_value: 0x54,
        name: "ArrowUpgrade10",
    },
    Item {
        rom_value: 0x4D,
        name: "ArrowUpgrade70",
    },
    Item {
        rom_value: 0x4E,
        name: "HalfMagic",
    },
    Item {
        rom_value: 0x4F,
        name: "QuarterMagic",
    },
    Item {
        rom_value: 0x55,
        name: "Programmable1",
    },
    Item {
        rom_value: 0x56,
        name: "Programmable2",
    },
    Item {
        rom_value: 0x57,
        name: "Programmable3",
    },
    Item {
        rom_value: 0x58,
        name: "SilverArrowUpgrade",
    },
    Item {
        rom_value: 0x59,
        name: "Rupoor",
    },
    Item {
        rom_value: 0x5B,
        name: "RedClock",
    },
    Item {
        rom_value: 0x5C,
        name: "BlueClock",
    },
    Item {
        rom_value: 0x5D,
        name: "GreenClock",
    },
    Item {
        rom_value: 0x5E,
        name: "ProgressiveSword",
    },
    Item {
        rom_value: 0x5F,
        name: "ProgressiveShield",
    },
    Item {
        rom_value: 0x60,
        name: "ProgressiveArmor",
    },
    Item {
        rom_value: 0x61,
        name: "ProgressiveGlove",
    },
    Item {
        rom_value: 0x62,
        name: "singleRNG",
    },
    Item {
        rom_value: 0x63,
        name: "multiRNG",
    },
    Item {
        rom_value: 0x64,
        name: "ProgressiveBow",
    },
    Item {
        rom_value: 0x65,
        name: "ProgressiveBowAlternate",
    },
    Item {
        rom_value: 0x6A,
        name: "Triforce",
    },
    Item {
        rom_value: 0x6B,
        name: "PowerStar",
    },
    Item {
        rom_value: 0x6C,
        name: "TriforcePiece",
    },
    Item {
        rom_value: 0x70,
        name: "MapLW",
    },
    Item {
        rom_value: 0x71,
        name: "MapDW",
    },
    Item {
        rom_value: 0x72,
        name: "MapA2",
    },
    Item {
        rom_value: 0x73,
        name: "MapD7",
    },
    Item {
        rom_value: 0x74,
        name: "MapD4",
    },
    Item {
        rom_value: 0x75,
        name: "MapP3",
    },
    Item {
        rom_value: 0x76,
        name: "MapD5",
    },
    Item {
        rom_value: 0x77,
        name: "MapD3",
    },
    Item {
        rom_value: 0x78,
        name: "MapD6",
    },
    Item {
        rom_value: 0x79,
        name: "MapD1",
    },
    Item {
        rom_value: 0x7A,
        name: "MapD2",
    },
    Item {
        rom_value: 0x7B,
        name: "MapA1",
    },
    Item {
        rom_value: 0x7C,
        name: "MapP2",
    },
    Item {
        rom_value: 0x7D,
        name: "MapP1",
    },
    Item {
        rom_value: 0x7E,
        name: "MapH1",
    },
    Item {
        rom_value: 0x7F,
        name: "MapH2",
    },
    Item {
        rom_value: 0x82,
        name: "CompassA2",
    },
    Item {
        rom_value: 0x83,
        name: "CompassD7",
    },
    Item {
        rom_value: 0x84,
        name: "CompassD4",
    },
    Item {
        rom_value: 0x85,
        name: "CompassP3",
    },
    Item {
        rom_value: 0x86,
        name: "CompassD5",
    },
    Item {
        rom_value: 0x87,
        name: "CompassD3",
    },
    Item {
        rom_value: 0x88,
        name: "CompassD6",
    },
    Item {
        rom_value: 0x89,
        name: "CompassD1",
    },
    Item {
        rom_value: 0x8A,
        name: "CompassD2",
    },
    Item {
        rom_value: 0x8B,
        name: "CompassA1",
    },
    Item {
        rom_value: 0x8C,
        name: "CompassP2",
    },
    Item {
        rom_value: 0x8D,
        name: "CompassP1",
    },
    Item {
        rom_value: 0x8E,
        name: "CompassH1",
    },
    Item {
        rom_value: 0x8F,
        name: "CompassH2",
    },
    Item {
        rom_value: 0x92,
        name: "BigKeyA2",
    },
    Item {
        rom_value: 0x93,
        name: "BigKeyD7",
    },
    Item {
        rom_value: 0x94,
        name: "BigKeyD4",
    },
    Item {
        rom_value: 0x95,
        name: "BigKeyP3",
    },
    Item {
        rom_value: 0x96,
        name: "BigKeyD5",
    },
    Item {
        rom_value: 0x97,
        name: "BigKeyD3",
    },
    Item {
        rom_value: 0x98,
        name: "BigKeyD6",
    },
    Item {
        rom_value: 0x99,
        name: "BigKeyD1",
    },
    Item {
        rom_value: 0x9A,
        name: "BigKeyD2",
    },
    Item {
        rom_value: 0x9B,
        name: "BigKeyA1",
    },
    Item {
        rom_value: 0x9C,
        name: "BigKeyP2",
    },
    Item {
        rom_value: 0x9D,
        name: "BigKeyP1",
    },
    Item {
        rom_value: 0x9E,
        name: "BigKeyH1",
    },
    Item {
        rom_value: 0x9F,
        name: "BigKeyH2",
    },
    Item {
        rom_value: 0xA0,
        name: "KeyH2",
    },
    Item {
        rom_value: 0xA1,
        name: "KeyH1",
    },
    Item {
        rom_value: 0xA2,
        name: "KeyP1",
    },
    Item {
        rom_value: 0xA3,
        name: "KeyP2",
    },
    Item {
        rom_value: 0xA4,
        name: "KeyA1",
    },
    Item {
        rom_value: 0xA5,
        name: "KeyD2",
    },
    Item {
        rom_value: 0xA6,
        name: "KeyD1",
    },
    Item {
        rom_value: 0xA7,
        name: "KeyD6",
    },
    Item {
        rom_value: 0xA8,
        name: "KeyD3",
    },
    Item {
        rom_value: 0xA9,
        name: "KeyD5",
    },
    Item {
        rom_value: 0xAA,
        name: "KeyP3",
    },
    Item {
        rom_value: 0xAB,
        name: "KeyD4",
    },
    Item {
        rom_value: 0xAC,
        name: "KeyD7",
    },
    Item {
        rom_value: 0xAD,
        name: "KeyA2",
    },
    Item {
        rom_value: 0xAF,
        name: "KeyGK",
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
