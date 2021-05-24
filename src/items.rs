pub struct Item {
    pub rom_value: u8,
    pub name: &'static str,
}

pub const UNKNOWN_ITEM: Item = Item {
    rom_value: 0x5A,
    name: "Unknown Item",
};

pub const L1SWORD: Item = Item {
    rom_value: 0x49,
    name: "L1Sword",
};

pub const L1SWORDANDSHIELD: Item = Item {
    rom_value: 0x00,
    name: "L1SwordAndShield",
};

pub const L2SWORD: Item = Item {
    rom_value: 0x01,
    name: "L2Sword",
};

pub const MASTERSWORD: Item = Item {
    rom_value: 0x50,
    name: "MasterSword",
};

pub const L3SWORD: Item = Item {
    rom_value: 0x02,
    name: "L3Sword",
};

pub const L4SWORD: Item = Item {
    rom_value: 0x03,
    name: "L4Sword",
};

pub const BLUESHIELD: Item = Item {
    rom_value: 0x04,
    name: "BlueShield",
};

pub const REDSHIELD: Item = Item {
    rom_value: 0x05,
    name: "RedShield",
};

pub const MIRRORSHIELD: Item = Item {
    rom_value: 0x06,
    name: "MirrorShield",
};

pub const FIREROD: Item = Item {
    rom_value: 0x07,
    name: "FireRod",
};

pub const ICEROD: Item = Item {
    rom_value: 0x08,
    name: "IceRod",
};

pub const HAMMER: Item = Item {
    rom_value: 0x09,
    name: "Hammer",
};

pub const HOOKSHOT: Item = Item {
    rom_value: 0x0A,
    name: "Hookshot",
};

pub const BOW: Item = Item {
    rom_value: 0x0B,
    name: "Bow",
};

pub const BOOMERANG: Item = Item {
    rom_value: 0x0C,
    name: "Boomerang",
};

pub const POWDER: Item = Item {
    rom_value: 0x0D,
    name: "Powder",
};

pub const BEE: Item = Item {
    rom_value: 0x0E,
    name: "Bee",
};

pub const LAMP: Item = Item {
    rom_value: 0x12,
    name: "Lamp",
};

pub const SHOVEL: Item = Item {
    rom_value: 0x13,
    name: "Shovel",
};

pub const OCARINAINACTIVE: Item = Item {
    rom_value: 0x14,
    name: "OcarinaInactive",
};

pub const CANEOFSOMARIA: Item = Item {
    rom_value: 0x15,
    name: "CaneOfSomaria",
};

pub const BOTTLE: Item = Item {
    rom_value: 0x16,
    name: "Bottle",
};

pub const PIECEOFHEART: Item = Item {
    rom_value: 0x17,
    name: "PieceOfHeart",
};

pub const CANEOFBYRNA: Item = Item {
    rom_value: 0x18,
    name: "CaneOfByrna",
};

pub const CAPE: Item = Item {
    rom_value: 0x19,
    name: "Cape",
};

pub const MAGICMIRROR: Item = Item {
    rom_value: 0x1A,
    name: "MagicMirror",
};

pub const POWERGLOVE: Item = Item {
    rom_value: 0x1B,
    name: "PowerGlove",
};

pub const TITANSMITT: Item = Item {
    rom_value: 0x1C,
    name: "TitansMitt",
};

pub const BOOKOFMUDORA: Item = Item {
    rom_value: 0x1D,
    name: "BookOfMudora",
};

pub const FLIPPERS: Item = Item {
    rom_value: 0x1E,
    name: "Flippers",
};

pub const MOONPEARL: Item = Item {
    rom_value: 0x1F,
    name: "MoonPearl",
};

pub const BUGCATCHINGNET: Item = Item {
    rom_value: 0x21,
    name: "BugCatchingNet",
};

pub const BLUEMAIL: Item = Item {
    rom_value: 0x22,
    name: "BlueMail",
};

pub const REDMAIL: Item = Item {
    rom_value: 0x23,
    name: "RedMail",
};

pub const KEY: Item = Item {
    rom_value: 0x24,
    name: "Key",
};

pub const COMPASS: Item = Item {
    rom_value: 0x25,
    name: "Compass",
};

pub const HEARTCONTAINERNOANIMATION: Item = Item {
    rom_value: 0x26,
    name: "HeartContainerNoAnimation",
};

pub const BOMB: Item = Item {
    rom_value: 0x27,
    name: "Bomb",
};

pub const THREEBOMBS: Item = Item {
    rom_value: 0x28,
    name: "ThreeBombs",
};

pub const MUSHROOM: Item = Item {
    rom_value: 0x29,
    name: "Mushroom",
};

pub const REDBOOMERANG: Item = Item {
    rom_value: 0x2A,
    name: "RedBoomerang",
};

pub const BOTTLEWITHREDPOTION: Item = Item {
    rom_value: 0x2B,
    name: "BottleWithRedPotion",
};

pub const BOTTLEWITHGREENPOTION: Item = Item {
    rom_value: 0x2C,
    name: "BottleWithGreenPotion",
};

pub const BOTTLEWITHBLUEPOTION: Item = Item {
    rom_value: 0x2D,
    name: "BottleWithBluePotion",
};

pub const REDPOTION: Item = Item {
    rom_value: 0x2E,
    name: "RedPotion",
};

pub const GREENPOTION: Item = Item {
    rom_value: 0x2F,
    name: "GreenPotion",
};

pub const BLUEPOTION: Item = Item {
    rom_value: 0x30,
    name: "BluePotion",
};

pub const TENBOMBS: Item = Item {
    rom_value: 0x31,
    name: "TenBombs",
};

pub const BIGKEY: Item = Item {
    rom_value: 0x32,
    name: "BigKey",
};

pub const MAP: Item = Item {
    rom_value: 0x33,
    name: "Map",
};

pub const ONERUPEE: Item = Item {
    rom_value: 0x34,
    name: "OneRupee",
};

pub const FIVERUPEES: Item = Item {
    rom_value: 0x35,
    name: "FiveRupees",
};

pub const TWENTYRUPEES: Item = Item {
    rom_value: 0x36,
    name: "TwentyRupees",
};

pub const BOWANDARROWS: Item = Item {
    rom_value: 0x3A,
    name: "BowAndArrows",
};

pub const BOWANDSILVERARROWS: Item = Item {
    rom_value: 0x3B,
    name: "BowAndSilverArrows",
};

pub const BOTTLEWITHBEE: Item = Item {
    rom_value: 0x3C,
    name: "BottleWithBee",
};

pub const BOTTLEWITHFAIRY: Item = Item {
    rom_value: 0x3D,
    name: "BottleWithFairy",
};

pub const BOSSHEARTCONTAINER: Item = Item {
    rom_value: 0x3E,
    name: "BossHeartContainer",
};

pub const HEARTCONTAINER: Item = Item {
    rom_value: 0x3F,
    name: "HeartContainer",
};

pub const ONEHUNDREDRUPEES: Item = Item {
    rom_value: 0x40,
    name: "OneHundredRupees",
};

pub const FIFTYRUPEES: Item = Item {
    rom_value: 0x41,
    name: "FiftyRupees",
};

pub const HEART: Item = Item {
    rom_value: 0x42,
    name: "Heart",
};

pub const ARROW: Item = Item {
    rom_value: 0x43,
    name: "Arrow",
};

pub const TENARROWS: Item = Item {
    rom_value: 0x44,
    name: "TenArrows",
};

pub const SMALLMAGIC: Item = Item {
    rom_value: 0x45,
    name: "SmallMagic",
};

pub const THREEHUNDREDRUPEES: Item = Item {
    rom_value: 0x46,
    name: "ThreeHundredRupees",
};

pub const TWENTYRUPEES2: Item = Item {
    rom_value: 0x47,
    name: "TwentyRupees2",
};

pub const BOTTLEWITHGOLDBEE: Item = Item {
    rom_value: 0x48,
    name: "BottleWithGoldBee",
};

pub const OCARINAACTIVE: Item = Item {
    rom_value: 0x4A,
    name: "OcarinaActive",
};

pub const PEGASUSBOOTS: Item = Item {
    rom_value: 0x4B,
    name: "PegasusBoots",
};

pub const BOMBUPGRADE5: Item = Item {
    rom_value: 0x51,
    name: "BombUpgrade5",
};

pub const BOMBUPGRADE10: Item = Item {
    rom_value: 0x52,
    name: "BombUpgrade10",
};

pub const BOMBUPGRADE50: Item = Item {
    rom_value: 0x4C,
    name: "BombUpgrade50",
};

pub const ARROWUPGRADE5: Item = Item {
    rom_value: 0x53,
    name: "ArrowUpgrade5",
};

pub const ARROWUPGRADE10: Item = Item {
    rom_value: 0x54,
    name: "ArrowUpgrade10",
};

pub const ARROWUPGRADE70: Item = Item {
    rom_value: 0x4D,
    name: "ArrowUpgrade70",
};

pub const HALFMAGIC: Item = Item {
    rom_value: 0x4E,
    name: "HalfMagic",
};

pub const QUARTERMAGIC: Item = Item {
    rom_value: 0x4F,
    name: "QuarterMagic",
};

pub const PROGRAMMABLE1: Item = Item {
    rom_value: 0x55,
    name: "Programmable1",
};

pub const PROGRAMMABLE2: Item = Item {
    rom_value: 0x56,
    name: "Programmable2",
};

pub const PROGRAMMABLE3: Item = Item {
    rom_value: 0x57,
    name: "Programmable3",
};

pub const SILVERARROWUPGRADE: Item = Item {
    rom_value: 0x58,
    name: "SilverArrowUpgrade",
};

pub const RUPOOR: Item = Item {
    rom_value: 0x59,
    name: "Rupoor",
};

pub const REDCLOCK: Item = Item {
    rom_value: 0x5B,
    name: "RedClock",
};

pub const BLUECLOCK: Item = Item {
    rom_value: 0x5C,
    name: "BlueClock",
};

pub const GREENCLOCK: Item = Item {
    rom_value: 0x5D,
    name: "GreenClock",
};

pub const PROGRESSIVESWORD: Item = Item {
    rom_value: 0x5E,
    name: "ProgressiveSword",
};

pub const PROGRESSIVESHIELD: Item = Item {
    rom_value: 0x5F,
    name: "ProgressiveShield",
};

pub const PROGRESSIVEARMOR: Item = Item {
    rom_value: 0x60,
    name: "ProgressiveArmor",
};

pub const PROGRESSIVEGLOVE: Item = Item {
    rom_value: 0x61,
    name: "ProgressiveGlove",
};

pub const SINGLERNG: Item = Item {
    rom_value: 0x62,
    name: "singleRNG",
};

pub const MULTIRNG: Item = Item {
    rom_value: 0x63,
    name: "multiRNG",
};

pub const PROGRESSIVEBOW: Item = Item {
    rom_value: 0x64,
    name: "ProgressiveBow",
};

pub const PROGRESSIVEBOWALTERNATE: Item = Item {
    rom_value: 0x65,
    name: "ProgressiveBowAlternate",
};

pub const TRIFORCE: Item = Item {
    rom_value: 0x6A,
    name: "Triforce",
};

pub const POWERSTAR: Item = Item {
    rom_value: 0x6B,
    name: "PowerStar",
};

pub const TRIFORCEPIECE: Item = Item {
    rom_value: 0x6C,
    name: "TriforcePiece",
};

pub const MAPLW: Item = Item {
    rom_value: 0x70,
    name: "MapLW",
};

pub const MAPDW: Item = Item {
    rom_value: 0x71,
    name: "MapDW",
};

pub const MAPA2: Item = Item {
    rom_value: 0x72,
    name: "MapA2",
};

pub const MAPD7: Item = Item {
    rom_value: 0x73,
    name: "MapD7",
};

pub const MAPD4: Item = Item {
    rom_value: 0x74,
    name: "MapD4",
};

pub const MAPP3: Item = Item {
    rom_value: 0x75,
    name: "MapP3",
};

pub const MAPD5: Item = Item {
    rom_value: 0x76,
    name: "MapD5",
};

pub const MAPD3: Item = Item {
    rom_value: 0x77,
    name: "MapD3",
};

pub const MAPD6: Item = Item {
    rom_value: 0x78,
    name: "MapD6",
};

pub const MAPD1: Item = Item {
    rom_value: 0x79,
    name: "MapD1",
};

pub const MAPD2: Item = Item {
    rom_value: 0x7A,
    name: "MapD2",
};

pub const MAPA1: Item = Item {
    rom_value: 0x7B,
    name: "MapA1",
};

pub const MAPP2: Item = Item {
    rom_value: 0x7C,
    name: "MapP2",
};

pub const MAPP1: Item = Item {
    rom_value: 0x7D,
    name: "MapP1",
};

pub const MAPH1: Item = Item {
    rom_value: 0x7E,
    name: "MapH1",
};

pub const MAPH2: Item = Item {
    rom_value: 0x7F,
    name: "MapH2",
};

pub const COMPASSA2: Item = Item {
    rom_value: 0x82,
    name: "CompassA2",
};

pub const COMPASSD7: Item = Item {
    rom_value: 0x83,
    name: "CompassD7",
};

pub const COMPASSD4: Item = Item {
    rom_value: 0x84,
    name: "CompassD4",
};

pub const COMPASSP3: Item = Item {
    rom_value: 0x85,
    name: "CompassP3",
};

pub const COMPASSD5: Item = Item {
    rom_value: 0x86,
    name: "CompassD5",
};

pub const COMPASSD3: Item = Item {
    rom_value: 0x87,
    name: "CompassD3",
};

pub const COMPASSD6: Item = Item {
    rom_value: 0x88,
    name: "CompassD6",
};

pub const COMPASSD1: Item = Item {
    rom_value: 0x89,
    name: "CompassD1",
};

pub const COMPASSD2: Item = Item {
    rom_value: 0x8A,
    name: "CompassD2",
};

pub const COMPASSA1: Item = Item {
    rom_value: 0x8B,
    name: "CompassA1",
};

pub const COMPASSP2: Item = Item {
    rom_value: 0x8C,
    name: "CompassP2",
};

pub const COMPASSP1: Item = Item {
    rom_value: 0x8D,
    name: "CompassP1",
};

pub const COMPASSH1: Item = Item {
    rom_value: 0x8E,
    name: "CompassH1",
};

pub const COMPASSH2: Item = Item {
    rom_value: 0x8F,
    name: "CompassH2",
};

pub const BIGKEYA2: Item = Item {
    rom_value: 0x92,
    name: "BigKeyA2",
};

pub const BIGKEYD7: Item = Item {
    rom_value: 0x93,
    name: "BigKeyD7",
};

pub const BIGKEYD4: Item = Item {
    rom_value: 0x94,
    name: "BigKeyD4",
};

pub const BIGKEYP3: Item = Item {
    rom_value: 0x95,
    name: "BigKeyP3",
};

pub const BIGKEYD5: Item = Item {
    rom_value: 0x96,
    name: "BigKeyD5",
};

pub const BIGKEYD3: Item = Item {
    rom_value: 0x97,
    name: "BigKeyD3",
};

pub const BIGKEYD6: Item = Item {
    rom_value: 0x98,
    name: "BigKeyD6",
};

pub const BIGKEYD1: Item = Item {
    rom_value: 0x99,
    name: "BigKeyD1",
};

pub const BIGKEYD2: Item = Item {
    rom_value: 0x9A,
    name: "BigKeyD2",
};

pub const BIGKEYA1: Item = Item {
    rom_value: 0x9B,
    name: "BigKeyA1",
};

pub const BIGKEYP2: Item = Item {
    rom_value: 0x9C,
    name: "BigKeyP2",
};

pub const BIGKEYP1: Item = Item {
    rom_value: 0x9D,
    name: "BigKeyP1",
};

pub const BIGKEYH1: Item = Item {
    rom_value: 0x9E,
    name: "BigKeyH1",
};

pub const BIGKEYH2: Item = Item {
    rom_value: 0x9F,
    name: "BigKeyH2",
};

pub const KEYH2: Item = Item {
    rom_value: 0xA0,
    name: "KeyH2",
};

pub const KEYH1: Item = Item {
    rom_value: 0xA1,
    name: "KeyH1",
};

pub const KEYP1: Item = Item {
    rom_value: 0xA2,
    name: "KeyP1",
};

pub const KEYP2: Item = Item {
    rom_value: 0xA3,
    name: "KeyP2",
};

pub const KEYA1: Item = Item {
    rom_value: 0xA4,
    name: "KeyA1",
};

pub const KEYD2: Item = Item {
    rom_value: 0xA5,
    name: "KeyD2",
};

pub const KEYD1: Item = Item {
    rom_value: 0xA6,
    name: "KeyD1",
};

pub const KEYD6: Item = Item {
    rom_value: 0xA7,
    name: "KeyD6",
};

pub const KEYD3: Item = Item {
    rom_value: 0xA8,
    name: "KeyD3",
};

pub const KEYD5: Item = Item {
    rom_value: 0xA9,
    name: "KeyD5",
};

pub const KEYP3: Item = Item {
    rom_value: 0xAA,
    name: "KeyP3",
};

pub const KEYD4: Item = Item {
    rom_value: 0xAB,
    name: "KeyD4",
};

pub const KEYD7: Item = Item {
    rom_value: 0xAC,
    name: "KeyD7",
};

pub const KEYA2: Item = Item {
    rom_value: 0xAD,
    name: "KeyA2",
};

pub const KEYGK: Item = Item {
    rom_value: 0xAF,
    name: "KeyGK",
};

pub const ITEMS: [&Item; 157] = [
    &UNKNOWN_ITEM,
    &L1SWORD,
    &L1SWORDANDSHIELD,
    &L2SWORD,
    &MASTERSWORD,
    &L3SWORD,
    &L4SWORD,
    &BLUESHIELD,
    &REDSHIELD,
    &MIRRORSHIELD,
    &FIREROD,
    &ICEROD,
    &HAMMER,
    &HOOKSHOT,
    &BOW,
    &BOOMERANG,
    &POWDER,
    &BEE,
    &LAMP,
    &SHOVEL,
    &OCARINAINACTIVE,
    &CANEOFSOMARIA,
    &BOTTLE,
    &PIECEOFHEART,
    &CANEOFBYRNA,
    &CAPE,
    &MAGICMIRROR,
    &POWERGLOVE,
    &TITANSMITT,
    &BOOKOFMUDORA,
    &FLIPPERS,
    &MOONPEARL,
    &BUGCATCHINGNET,
    &BLUEMAIL,
    &REDMAIL,
    &KEY,
    &COMPASS,
    &HEARTCONTAINERNOANIMATION,
    &BOMB,
    &THREEBOMBS,
    &MUSHROOM,
    &REDBOOMERANG,
    &BOTTLEWITHREDPOTION,
    &BOTTLEWITHGREENPOTION,
    &BOTTLEWITHBLUEPOTION,
    &REDPOTION,
    &GREENPOTION,
    &BLUEPOTION,
    &TENBOMBS,
    &BIGKEY,
    &MAP,
    &ONERUPEE,
    &FIVERUPEES,
    &TWENTYRUPEES,
    &BOWANDARROWS,
    &BOWANDSILVERARROWS,
    &BOTTLEWITHBEE,
    &BOTTLEWITHFAIRY,
    &BOSSHEARTCONTAINER,
    &HEARTCONTAINER,
    &ONEHUNDREDRUPEES,
    &FIFTYRUPEES,
    &HEART,
    &ARROW,
    &TENARROWS,
    &SMALLMAGIC,
    &THREEHUNDREDRUPEES,
    &TWENTYRUPEES2,
    &BOTTLEWITHGOLDBEE,
    &OCARINAACTIVE,
    &PEGASUSBOOTS,
    &BOMBUPGRADE5,
    &BOMBUPGRADE10,
    &BOMBUPGRADE50,
    &ARROWUPGRADE5,
    &ARROWUPGRADE10,
    &ARROWUPGRADE70,
    &HALFMAGIC,
    &QUARTERMAGIC,
    &PROGRAMMABLE1,
    &PROGRAMMABLE2,
    &PROGRAMMABLE3,
    &SILVERARROWUPGRADE,
    &RUPOOR,
    &REDCLOCK,
    &BLUECLOCK,
    &GREENCLOCK,
    &PROGRESSIVESWORD,
    &PROGRESSIVESHIELD,
    &PROGRESSIVEARMOR,
    &PROGRESSIVEGLOVE,
    &SINGLERNG,
    &MULTIRNG,
    &PROGRESSIVEBOW,
    &PROGRESSIVEBOWALTERNATE,
    &TRIFORCE,
    &POWERSTAR,
    &TRIFORCEPIECE,
    &MAPLW,
    &MAPDW,
    &MAPA2,
    &MAPD7,
    &MAPD4,
    &MAPP3,
    &MAPD5,
    &MAPD3,
    &MAPD6,
    &MAPD1,
    &MAPD2,
    &MAPA1,
    &MAPP2,
    &MAPP1,
    &MAPH1,
    &MAPH2,
    &COMPASSA2,
    &COMPASSD7,
    &COMPASSD4,
    &COMPASSP3,
    &COMPASSD5,
    &COMPASSD3,
    &COMPASSD6,
    &COMPASSD1,
    &COMPASSD2,
    &COMPASSA1,
    &COMPASSP2,
    &COMPASSP1,
    &COMPASSH1,
    &COMPASSH2,
    &BIGKEYA2,
    &BIGKEYD7,
    &BIGKEYD4,
    &BIGKEYP3,
    &BIGKEYD5,
    &BIGKEYD3,
    &BIGKEYD6,
    &BIGKEYD1,
    &BIGKEYD2,
    &BIGKEYA1,
    &BIGKEYP2,
    &BIGKEYP1,
    &BIGKEYH1,
    &BIGKEYH2,
    &KEYH2,
    &KEYH1,
    &KEYP1,
    &KEYP2,
    &KEYA1,
    &KEYD2,
    &KEYD1,
    &KEYD6,
    &KEYD3,
    &KEYD5,
    &KEYP3,
    &KEYD4,
    &KEYD7,
    &KEYA2,
    &KEYGK,
];

pub fn get_item(rom_value: u8) -> &'static Item {
    for i in &ITEMS {
        if i.rom_value == rom_value {
            return i;
        }
    }

    return ITEMS[0];
}
