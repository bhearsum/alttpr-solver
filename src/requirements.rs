use crate::items::{FIREROD, LAMP, PROGRESSIVEGLOVE, Item};
use std::fmt;

pub struct Requirements {
    items: &'static [&'static Item],
}


macro_rules! reqs {
    ($($item:expr),*) => {
        Requirements {
            items: &[
                $($item),*,
            ]
        }
    };
}

    //&PROGRESSIVEGLOVE && (&LAMP || &FIREROD)
    //&PROGRESSIVEGLOVE && &LAMP && &FIREROD
pub const DESERT_PALACE_BOSS_REQUIREMENTS: Requirements = reqs!(
    &PROGRESSIVEGLOVE && (&LAMP || &FIREROD)
);


// things we need to handle:
// - nested boolean logic with && and ||, and parenthesis
// - abscence of items
// - items being in a specific location
// - TODO: SCAN FOR MORE THINGS
//
// you really need to decide what you want the requirements to look like
// and how you want to parse them
// Ideally we want to write things like:
// &PROGRESSIVEGLOVE && (&LAMP || &FIREROD)
// and turn them into things like:
// &[&[&PROGRESSIVEGLOVE, &LAMP], &[PROGRESSIVEGLOVE, &FIREROD]]
//
// &PROGRESSIVEGLOVE && &PROGRESSIVEGLOVE && &HAMMER &&
// (&HOOKSHOT || (
//     (&CANEOFBYRNA || &CAPE) && (
//         &KEYD5 && (
//             !BIGKEYD5 || ItemIn { item: &BIGKEYD5, locs: &[&ICE_PALACE_MAP_CHEST, &ICE_PALACE_SPIKE_ROOM] }
//         )
//     )
// )
//
// turns into:
// &[
//     &[&PROGRESSIVEGLOVE, &PROGRESSIVEGLOVE, &HAMMER, &HOOKSHOT],
//     &[&PROGRESSIVEGLOVE, &PROGRESSIVEGLOVE, &HAMMER, &CANEOFBYRNA, &KEYD5, AbsentItem(&BIGKEYD5)],,
//     &[&PROGRESSIVEGLOVE, &PROGRESSIVEGLOVE, &HAMMER, &CAPE, &KEYD5, AbsentItem(&BIGKEYD5)],
//     &[&PROGRESSIVEGLOVE, &PROGRESSIVEGLOVE, &HAMMER, &CANEOFBYRNA, &KEYD5, ItemIn(...)],
//     &[&PROGRESSIVEGLOVE, &PROGRESSIVEGLOVE, &HAMMER, &CAPE, &KEYD5, ItemIn(...)],
//  ],
