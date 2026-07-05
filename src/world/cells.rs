use crate::cell::{
    CellColor, CellData, CellDataFire, CellDataGas, CellDataGranular, CellDataLiquid,
    CellDataStatic, CellDataType,
};
pub static CELLS: [CellData; 10] = [
    //0
    CellData {
        color: CellColor::AIR,
        name: "air",
        type_data: CellDataType::Air,
        density: 0,
    },
    //1
    CellData {
        color: CellColor::new(0xff, 0x6a, 0x00, 0xff),
        name: "fire",
        type_data: CellDataType::Fire(CellDataFire {}),
        density: 1,
    },
    //2
    CellData {
        color: CellColor::new(0x2f, 0x8f, 0xff, 0xff),
        name: "water",
        type_data: CellDataType::Liquid(CellDataLiquid {}),
        density: 64,
    },
    //3
    CellData {
        color: CellColor::new(0xd8, 0xd8, 0xd8, 0xff),
        name: "steam",
        type_data: CellDataType::Gas(CellDataGas {}),
        density: 32,
    },
    //4
    CellData {
        color: CellColor::new(0xb8, 0xf0, 0xff, 0xff),
        name: "ice",
        type_data: CellDataType::Static(CellDataStatic {}),
        density: 255,
    },
    //5
    CellData {
        color: CellColor::new(0x8b, 0x5a, 0x2b, 0xff),
        name: "dirt",
        type_data: CellDataType::Static(CellDataStatic {}),
        density: 255,
    },
    //6
    CellData {
        color: CellColor::new(0x7a, 0x7a, 0x7a, 0xff),
        name: "stone",
        type_data: CellDataType::Static(CellDataStatic {}),
        density: 255,
    },
    //7
    CellData {
        color: CellColor::new(0xc8, 0xc8, 0xd0, 0xff),
        name: "iron",
        type_data: CellDataType::Static(CellDataStatic {}),
        density: 255,
    },
    //8
    CellData {
        color: CellColor::new(0x6b, 0x44, 0x22, 0xff),
        name: "wood",
        type_data: CellDataType::Static(CellDataStatic {}),
        density: 255,
    },
    //9
    CellData {
        color: CellColor::new(0xcb, 0xbd, 0x93, 0xff),
        name: "sand",
        type_data: CellDataType::Granular(CellDataGranular {}),
        density: 128,
    },
];
