use crate::cell::{CellColor, CellData};
pub const CELLS: [CellData; 9] = [
    CellData {
        id: 0,
        color: CellColor::new(0x00, 0x00, 0x00, 0x00),
        name: "air",
    },
    CellData {
        id: 1,
        color: CellColor::new(0xff, 0x6a, 0x00, 0xff),
        name: "fire",
    },
    CellData {
        id: 2,
        color: CellColor::new(0x2f, 0x8f, 0xff, 0xff),
        name: "water",
    },
    CellData {
        id: 3,
        color: CellColor::new(0xd8, 0xd8, 0xd8, 0xff),
        name: "steam",
    },
    CellData {
        id: 4,
        color: CellColor::new(0xb8, 0xf0, 0xff, 0xff),
        name: "ice",
    },
    CellData {
        id: 5,
        color: CellColor::new(0x8b, 0x5a, 0x2b, 0xff),
        name: "dirt",
    },
    CellData {
        id: 6,
        color: CellColor::new(0x7a, 0x7a, 0x7a, 0xff),
        name: "stone",
    },
    CellData {
        id: 7,
        color: CellColor::new(0xc8, 0xc8, 0xd0, 0xff),
        name: "iron",
    },
    CellData {
        id: 8,
        color: CellColor::new(0x6b, 0x44, 0x22, 0xff),
        name: "wood",
    },
];
