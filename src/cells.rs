use crate::cell::{
    CellColor, CellData, CellDataFire, CellDataGas, CellDataGranular, CellDataLiquid,
    CellDataStatic, CellDataType,
};
pub const CELLS: [CellData; 10] = [
    CellData {
        id: 0,
        color: CellColor::AIR,
        name: "air",
        type_data: CellDataType::Air,
    },
    CellData {
        id: 1,
        color: CellColor::new(0xff, 0x6a, 0x00, 0xff),
        name: "fire",
        type_data: CellDataType::Fire(CellDataFire {}),
    },
    CellData {
        id: 2,
        color: CellColor::new(0x2f, 0x8f, 0xff, 0xff),
        name: "water",
        type_data: CellDataType::Liquid(CellDataLiquid {}),
    },
    CellData {
        id: 3,
        color: CellColor::new(0xd8, 0xd8, 0xd8, 0xff),
        name: "steam",
        type_data: CellDataType::Gas(CellDataGas {}),
    },
    CellData {
        id: 4,
        color: CellColor::new(0xb8, 0xf0, 0xff, 0xff),
        name: "ice",
        type_data: CellDataType::Static(CellDataStatic {}),
    },
    CellData {
        id: 5,
        color: CellColor::new(0x8b, 0x5a, 0x2b, 0xff),
        name: "dirt",
        type_data: CellDataType::Static(CellDataStatic {}),
    },
    CellData {
        id: 6,
        color: CellColor::new(0x7a, 0x7a, 0x7a, 0xff),
        name: "stone",
        type_data: CellDataType::Static(CellDataStatic {}),
    },
    CellData {
        id: 7,
        color: CellColor::new(0xc8, 0xc8, 0xd0, 0xff),
        name: "iron",
        type_data: CellDataType::Static(CellDataStatic {}),
    },
    CellData {
        id: 8,
        color: CellColor::new(0x6b, 0x44, 0x22, 0xff),
        name: "wood",
        type_data: CellDataType::Static(CellDataStatic {}),
    },
    CellData {
        id: 9,
        color: CellColor::new(0xcb, 0xbd, 0x93, 0xff),
        name: "sand",
        type_data: CellDataType::Granular(CellDataGranular {}),
    },
];
