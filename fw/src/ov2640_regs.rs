pub struct RegisterData {
    pub reg: u8,
    pub val: u8,
}

pub const OV2640_JPEG_INIT: &[RegisterData] = &[
    RegisterData {
        reg: 0xff,
        val: 0x00,
    }, // Bank Select 0 (DSP/Sensor)
    RegisterData {
        reg: 0x2c,
        val: 0xff,
    },
    RegisterData {
        reg: 0x2e,
        val: 0xdf,
    },
    RegisterData {
        reg: 0xff,
        val: 0x01,
    }, // Bank Select 1 (Sensor)
    RegisterData {
        reg: 0x3c,
        val: 0x32,
    }, // COM12
    RegisterData {
        reg: 0x11,
        val: 0x00,
    }, // CLKRC
    RegisterData {
        reg: 0x09,
        val: 0x02,
    }, // COM2
    RegisterData {
        reg: 0x04,
        val: 0x28,
    }, // REG04
    RegisterData {
        reg: 0x13,
        val: 0xe5,
    }, // COM8
    RegisterData {
        reg: 0x14,
        val: 0x48,
    }, // COM9 (AGC Ceiling)
    RegisterData {
        reg: 0x2c,
        val: 0x0c,
    },
    RegisterData {
        reg: 0x33,
        val: 0x78,
    },
    RegisterData {
        reg: 0x3a,
        val: 0x33,
    },
    RegisterData {
        reg: 0x3b,
        val: 0xfB,
    },
    RegisterData {
        reg: 0x3e,
        val: 0x00,
    },
    RegisterData {
        reg: 0x43,
        val: 0x11,
    },
    RegisterData {
        reg: 0x16,
        val: 0x10,
    },
    RegisterData {
        reg: 0x39,
        val: 0x92,
    },
    RegisterData {
        reg: 0x35,
        val: 0xda,
    },
    RegisterData {
        reg: 0x22,
        val: 0x1a,
    },
    RegisterData {
        reg: 0x37,
        val: 0xc3,
    },
    RegisterData {
        reg: 0x23,
        val: 0x00,
    },
    RegisterData {
        reg: 0x34,
        val: 0xc0,
    },
    RegisterData {
        reg: 0x36,
        val: 0x1a,
    },
    RegisterData {
        reg: 0x06,
        val: 0x88,
    },
    RegisterData {
        reg: 0x07,
        val: 0xc0,
    },
    RegisterData {
        reg: 0x0d,
        val: 0x87,
    },
    RegisterData {
        reg: 0x0e,
        val: 0x41,
    },
    RegisterData {
        reg: 0x4c,
        val: 0x00,
    },
    RegisterData {
        reg: 0x48,
        val: 0x00,
    },
    RegisterData {
        reg: 0x5B,
        val: 0x00,
    },
    RegisterData {
        reg: 0x42,
        val: 0x03,
    },
    RegisterData {
        reg: 0x4a,
        val: 0x81,
    },
    RegisterData {
        reg: 0x21,
        val: 0x99,
    },
    RegisterData {
        reg: 0x24,
        val: 0x40,
    },
    RegisterData {
        reg: 0x25,
        val: 0x38,
    },
    RegisterData {
        reg: 0x26,
        val: 0x82,
    },
    RegisterData {
        reg: 0x5c,
        val: 0x00,
    },
    RegisterData {
        reg: 0x63,
        val: 0x00,
    },
    RegisterData {
        reg: 0x61,
        val: 0x70,
    },
    RegisterData {
        reg: 0x62,
        val: 0x80,
    },
    RegisterData {
        reg: 0x7c,
        val: 0x05,
    },
    RegisterData {
        reg: 0x20,
        val: 0x80,
    },
    RegisterData {
        reg: 0x28,
        val: 0x30,
    },
    RegisterData {
        reg: 0x6c,
        val: 0x00,
    },
    RegisterData {
        reg: 0x6d,
        val: 0x80,
    },
    RegisterData {
        reg: 0x6e,
        val: 0x00,
    },
    RegisterData {
        reg: 0x70,
        val: 0x02,
    },
    RegisterData {
        reg: 0x71,
        val: 0x94,
    },
    RegisterData {
        reg: 0x73,
        val: 0xc1,
    },
    RegisterData {
        reg: 0x12,
        val: 0x40,
    },
    RegisterData {
        reg: 0x17,
        val: 0x11,
    },
    RegisterData {
        reg: 0x18,
        val: 0x43,
    },
    RegisterData {
        reg: 0x19,
        val: 0x00,
    },
    RegisterData {
        reg: 0x1a,
        val: 0x4b,
    },
    RegisterData {
        reg: 0x32,
        val: 0x09,
    },
    RegisterData {
        reg: 0x37,
        val: 0xc0,
    },
    RegisterData {
        reg: 0x4f,
        val: 0x60,
    },
    RegisterData {
        reg: 0x50,
        val: 0xa8,
    },
    RegisterData {
        reg: 0x6d,
        val: 0x00,
    },
    RegisterData {
        reg: 0x3d,
        val: 0x38,
    },
    RegisterData {
        reg: 0x46,
        val: 0x3f,
    },
    RegisterData {
        reg: 0x4f,
        val: 0x60,
    },
    RegisterData {
        reg: 0x0c,
        val: 0x3c,
    },
    RegisterData {
        reg: 0xff,
        val: 0x00,
    },
    RegisterData {
        reg: 0xe5,
        val: 0x7f,
    },
    RegisterData {
        reg: 0xf9,
        val: 0xc0,
    },
    RegisterData {
        reg: 0x41,
        val: 0x24,
    },
    RegisterData {
        reg: 0xe0,
        val: 0x14,
    },
    RegisterData {
        reg: 0x76,
        val: 0xff,
    },
    RegisterData {
        reg: 0x33,
        val: 0xa0,
    },
    RegisterData {
        reg: 0x42,
        val: 0x20,
    },
    RegisterData {
        reg: 0x43,
        val: 0x18,
    },
    RegisterData {
        reg: 0x4c,
        val: 0x00,
    },
    RegisterData {
        reg: 0x87,
        val: 0xd5,
    },
    RegisterData {
        reg: 0x88,
        val: 0x3f,
    },
    RegisterData {
        reg: 0xd7,
        val: 0x03,
    },
    RegisterData {
        reg: 0xd9,
        val: 0x10,
    },
    RegisterData {
        reg: 0xd3,
        val: 0x82,
    },
    RegisterData {
        reg: 0xc8,
        val: 0x08,
    },
    RegisterData {
        reg: 0xc9,
        val: 0x80,
    },
    RegisterData {
        reg: 0x7c,
        val: 0x00,
    },
    RegisterData {
        reg: 0x7d,
        val: 0x00,
    },
    RegisterData {
        reg: 0x7c,
        val: 0x03,
    },
    RegisterData {
        reg: 0x7d,
        val: 0x48,
    },
    RegisterData {
        reg: 0x7d,
        val: 0x48,
    },
    RegisterData {
        reg: 0x7c,
        val: 0x08,
    },
    RegisterData {
        reg: 0x7d,
        val: 0x20,
    },
    RegisterData {
        reg: 0x7d,
        val: 0x10,
    },
    RegisterData {
        reg: 0x7d,
        val: 0x0e,
    },
    RegisterData {
        reg: 0x90,
        val: 0x00,
    },
    RegisterData {
        reg: 0x91,
        val: 0x0e,
    },
    RegisterData {
        reg: 0x91,
        val: 0x1a,
    },
    RegisterData {
        reg: 0x91,
        val: 0x31,
    },
    RegisterData {
        reg: 0x91,
        val: 0x5a,
    },
    RegisterData {
        reg: 0x91,
        val: 0x69,
    },
    RegisterData {
        reg: 0x91,
        val: 0x75,
    },
    RegisterData {
        reg: 0x91,
        val: 0x7e,
    },
    RegisterData {
        reg: 0x91,
        val: 0x88,
    },
    RegisterData {
        reg: 0x91,
        val: 0x8f,
    },
    RegisterData {
        reg: 0x91,
        val: 0x96,
    },
    RegisterData {
        reg: 0x91,
        val: 0xa3,
    },
    RegisterData {
        reg: 0x91,
        val: 0xaf,
    },
    RegisterData {
        reg: 0x91,
        val: 0xc4,
    },
    RegisterData {
        reg: 0x91,
        val: 0xd7,
    },
    RegisterData {
        reg: 0x91,
        val: 0xe8,
    },
    RegisterData {
        reg: 0x91,
        val: 0x20,
    },
    RegisterData {
        reg: 0x92,
        val: 0x00,
    },
    RegisterData {
        reg: 0x93,
        val: 0x06,
    },
    RegisterData {
        reg: 0x93,
        val: 0xe3,
    },
    RegisterData {
        reg: 0x93,
        val: 0x05,
    },
    RegisterData {
        reg: 0x93,
        val: 0x05,
    },
    RegisterData {
        reg: 0x93,
        val: 0x00,
    },
    RegisterData {
        reg: 0x93,
        val: 0x04,
    },
    RegisterData {
        reg: 0x93,
        val: 0x00,
    },
    RegisterData {
        reg: 0x93,
        val: 0x00,
    },
    RegisterData {
        reg: 0x93,
        val: 0x00,
    },
    RegisterData {
        reg: 0x93,
        val: 0x00,
    },
    RegisterData {
        reg: 0x93,
        val: 0x00,
    },
    RegisterData {
        reg: 0x93,
        val: 0x00,
    },
    RegisterData {
        reg: 0x93,
        val: 0x00,
    },
    RegisterData {
        reg: 0x96,
        val: 0x00,
    },
    RegisterData {
        reg: 0x97,
        val: 0x08,
    },
    RegisterData {
        reg: 0x97,
        val: 0x19,
    },
    RegisterData {
        reg: 0x97,
        val: 0x02,
    },
    RegisterData {
        reg: 0x97,
        val: 0x0c,
    },
    RegisterData {
        reg: 0x97,
        val: 0x24,
    },
    RegisterData {
        reg: 0x97,
        val: 0x30,
    },
    RegisterData {
        reg: 0x97,
        val: 0x28,
    },
    RegisterData {
        reg: 0x97,
        val: 0x26,
    },
    RegisterData {
        reg: 0x97,
        val: 0x02,
    },
    RegisterData {
        reg: 0x97,
        val: 0x98,
    },
    RegisterData {
        reg: 0x97,
        val: 0x80,
    },
    RegisterData {
        reg: 0x97,
        val: 0x00,
    },
    RegisterData {
        reg: 0x97,
        val: 0x00,
    },
    RegisterData {
        reg: 0xc3,
        val: 0xed,
    },
    RegisterData {
        reg: 0xa4,
        val: 0x00,
    },
    RegisterData {
        reg: 0xa8,
        val: 0x00,
    },
    RegisterData {
        reg: 0xc5,
        val: 0x11,
    },
    RegisterData {
        reg: 0xc6,
        val: 0x51,
    },
    RegisterData {
        reg: 0xbf,
        val: 0x80,
    },
    RegisterData {
        reg: 0xc7,
        val: 0x10,
    },
    RegisterData {
        reg: 0xb6,
        val: 0x66,
    },
    RegisterData {
        reg: 0xb8,
        val: 0xA5,
    },
    RegisterData {
        reg: 0xb7,
        val: 0x64,
    },
    RegisterData {
        reg: 0xb9,
        val: 0x7C,
    },
    RegisterData {
        reg: 0xb3,
        val: 0xaf,
    },
    RegisterData {
        reg: 0xb4,
        val: 0x97,
    },
    RegisterData {
        reg: 0xb5,
        val: 0xFF,
    },
    RegisterData {
        reg: 0xb0,
        val: 0xC5,
    },
    RegisterData {
        reg: 0xb1,
        val: 0x94,
    },
    RegisterData {
        reg: 0xb2,
        val: 0x0f,
    },
    RegisterData {
        reg: 0xc4,
        val: 0x5c,
    },
    RegisterData {
        reg: 0xc0,
        val: 0x64,
    },
    RegisterData {
        reg: 0xc1,
        val: 0x4B,
    },
    RegisterData {
        reg: 0x8c,
        val: 0x00,
    },
    RegisterData {
        reg: 0x86,
        val: 0x3D,
    },
    RegisterData {
        reg: 0x50,
        val: 0x00,
    },
    RegisterData {
        reg: 0x51,
        val: 0xC8,
    },
    RegisterData {
        reg: 0x52,
        val: 0x96,
    },
    RegisterData {
        reg: 0x53,
        val: 0x00,
    },
    RegisterData {
        reg: 0x54,
        val: 0x00,
    },
    RegisterData {
        reg: 0x55,
        val: 0x00,
    },
    RegisterData {
        reg: 0x5a,
        val: 0xC8,
    },
    RegisterData {
        reg: 0x5b,
        val: 0x96,
    },
    RegisterData {
        reg: 0x5c,
        val: 0x00,
    },
    RegisterData {
        reg: 0xd3,
        val: 0x00,
    }, //{ 0xd3, 0x7f },
    RegisterData {
        reg: 0xc3,
        val: 0xed,
    },
    RegisterData {
        reg: 0x7f,
        val: 0x00,
    },
    RegisterData {
        reg: 0xda,
        val: 0x00,
    },
    RegisterData {
        reg: 0xe5,
        val: 0x1f,
    },
    RegisterData {
        reg: 0xe1,
        val: 0x67,
    },
    RegisterData {
        reg: 0xe0,
        val: 0x00,
    },
    RegisterData {
        reg: 0xdd,
        val: 0x7f,
    },
    RegisterData {
        reg: 0x05,
        val: 0x00,
    },
    RegisterData {
        reg: 0x12,
        val: 0x40,
    },
    RegisterData {
        reg: 0xd3,
        val: 0x04,
    }, //{ 0xd3, 0x7f },
    RegisterData {
        reg: 0xc0,
        val: 0x16,
    },
    RegisterData {
        reg: 0xC1,
        val: 0x12,
    },
    RegisterData {
        reg: 0x8c,
        val: 0x00,
    },
    RegisterData {
        reg: 0x86,
        val: 0x3d,
    },
    RegisterData {
        reg: 0x50,
        val: 0x00,
    },
    RegisterData {
        reg: 0x51,
        val: 0x2C,
    },
    RegisterData {
        reg: 0x52,
        val: 0x24,
    },
    RegisterData {
        reg: 0x53,
        val: 0x00,
    },
    RegisterData {
        reg: 0x54,
        val: 0x00,
    },
    RegisterData {
        reg: 0x55,
        val: 0x00,
    },
    RegisterData {
        reg: 0x5A,
        val: 0x10,
    },
    RegisterData {
        reg: 0x5b,
        val: 0x10,
    },
    RegisterData {
        reg: 0x5c,
        val: 0x00,
    },
    RegisterData {
        reg: 0xff,
        val: 0xff,
    },
];

pub const OV2640_YUV422: &[RegisterData] = &[
    RegisterData {
        reg: 0xFF,
        val: 0x00,
    },
    RegisterData {
        reg: 0x05,
        val: 0x00,
    },
    RegisterData {
        reg: 0xDA,
        val: 0x10,
    },
    RegisterData {
        reg: 0xD7,
        val: 0x03,
    },
    RegisterData {
        reg: 0xDF,
        val: 0x00,
    },
    RegisterData {
        reg: 0x33,
        val: 0x80,
    }, // DSP_CTRL3
    RegisterData {
        reg: 0x3C,
        val: 0x40,
    }, // COM12
    RegisterData {
        reg: 0xe1,
        val: 0x77,
    },
    RegisterData {
        reg: 0x00,
        val: 0x00,
    },
    RegisterData {
        reg: 0xff,
        val: 0xff,
    },
];

pub const OV2640_JPEG: &[RegisterData] = &[
    RegisterData {
        reg: 0xe0,
        val: 0x14,
    },
    RegisterData {
        reg: 0xe1,
        val: 0x77,
    },
    RegisterData {
        reg: 0xe5,
        val: 0x1f,
    },
    RegisterData {
        reg: 0xd7,
        val: 0x03,
    },
    RegisterData {
        reg: 0xda,
        val: 0x10,
    },
    RegisterData {
        reg: 0xe0,
        val: 0x00,
    },
    RegisterData {
        reg: 0xFF,
        val: 0x01,
    }, // Bank 1
    RegisterData {
        reg: 0x04,
        val: 0x08,
    },
    RegisterData {
        reg: 0xff,
        val: 0xff,
    },
];

pub const OV2640_320X240_JPEG: &[RegisterData] = &[
    RegisterData {
        reg: 0xff,
        val: 0x01,
    }, // Bank 1
    RegisterData {
        reg: 0x12,
        val: 0x40,
    },
    RegisterData {
        reg: 0x17,
        val: 0x11,
    },
    RegisterData {
        reg: 0x18,
        val: 0x43,
    },
    RegisterData {
        reg: 0x19,
        val: 0x00,
    },
    RegisterData {
        reg: 0x1a,
        val: 0x4b,
    },
    RegisterData {
        reg: 0x32,
        val: 0x09,
    },
    RegisterData {
        reg: 0x4f,
        val: 0xca,
    },
    RegisterData {
        reg: 0x50,
        val: 0xa8,
    },
    RegisterData {
        reg: 0x5a,
        val: 0x23,
    },
    RegisterData {
        reg: 0x6d,
        val: 0x00,
    },
    RegisterData {
        reg: 0x39,
        val: 0x12,
    },
    RegisterData {
        reg: 0x35,
        val: 0xda,
    },
    RegisterData {
        reg: 0x22,
        val: 0x1a,
    },
    RegisterData {
        reg: 0x37,
        val: 0xc3,
    },
    RegisterData {
        reg: 0x23,
        val: 0x00,
    },
    RegisterData {
        reg: 0x34,
        val: 0xc0,
    },
    RegisterData {
        reg: 0x36,
        val: 0x1a,
    },
    RegisterData {
        reg: 0x06,
        val: 0x88,
    },
    RegisterData {
        reg: 0x07,
        val: 0xc0,
    },
    RegisterData {
        reg: 0x0d,
        val: 0x87,
    },
    RegisterData {
        reg: 0x0e,
        val: 0x41,
    },
    RegisterData {
        reg: 0x4c,
        val: 0x00,
    },
    RegisterData {
        reg: 0xff,
        val: 0x00,
    },
    RegisterData {
        reg: 0xe0,
        val: 0x04,
    },
    RegisterData {
        reg: 0xc0,
        val: 0x64,
    },
    RegisterData {
        reg: 0xc1,
        val: 0x4b,
    },
    RegisterData {
        reg: 0x86,
        val: 0x35,
    },
    RegisterData {
        reg: 0x50,
        val: 0x89,
    },
    RegisterData {
        reg: 0x51,
        val: 0xc8,
    },
    RegisterData {
        reg: 0x52,
        val: 0x96,
    },
    RegisterData {
        reg: 0x53,
        val: 0x00,
    },
    RegisterData {
        reg: 0x54,
        val: 0x00,
    },
    RegisterData {
        reg: 0x55,
        val: 0x00,
    },
    RegisterData {
        reg: 0x57,
        val: 0x00,
    },
    RegisterData {
        reg: 0x5a,
        val: 0x50,
    },
    RegisterData {
        reg: 0x5b,
        val: 0x3c,
    },
    RegisterData {
        reg: 0x5c,
        val: 0x00,
    },
    RegisterData {
        reg: 0xe0,
        val: 0x00,
    },
    RegisterData {
        reg: 0xff,
        val: 0xff,
    },
];

pub const OV2640_320X240_ROI_JPEG: &[RegisterData] = &[
    RegisterData {
        reg: 0xff,
        val: 0x01,
    }, // Bank 1
    RegisterData {
        reg: 0x12,
        val: 0x00,
    }, // COM7: UXGA Mode (No Subsample)
    // Horizontal Window: Center 320 in 1600 (640-960)
    // Unit: 2 pixels. Start=320(0x140), End=480(0x1E0)
    // REG32[2:0] = 0, REG32[5:3] = 0
    RegisterData {
        reg: 0x32,
        val: 0x00,
    },
    RegisterData {
        reg: 0x17,
        val: 0x28,
    }, // HREFST
    RegisterData {
        reg: 0x18,
        val: 0x3c,
    }, // HREFEND
    // Vertical Window: Center 240 in 1200 (480-720)
    // Unit: 2 lines. Start=240(0xF0), End=360(0x168)
    // COM1[1:0] = 0, COM1[3:2] = 0 (+ default bits?)
    // Note: COM1 might have other bits. Default is often 0.
    // Safety: just set to 0 as calc requires.
    RegisterData {
        reg: 0x03,
        val: 0x00,
    },
    RegisterData {
        reg: 0x19,
        val: 0x3c,
    }, // VSTRT
    RegisterData {
        reg: 0x1a,
        val: 0x5a,
    }, // VEND
    // DSP Output Size (320x240)
    RegisterData {
        reg: 0x4f,
        val: 0xca,
    },
    RegisterData {
        reg: 0x50,
        val: 0xa8,
    },
    RegisterData {
        reg: 0x5a,
        val: 0x50,
    }, // ZMOW (320/4 = 80 = 0x50)
    RegisterData {
        reg: 0x5b,
        val: 0x3c,
    }, // ZMOH (240/4 = 60 = 0x3c)
    RegisterData {
        reg: 0x5c,
        val: 0x00,
    },
    // Other settings from generic 320x240
    RegisterData {
        reg: 0x6d,
        val: 0x00,
    },
    RegisterData {
        reg: 0x39,
        val: 0x12,
    },
    RegisterData {
        reg: 0x35,
        val: 0xda,
    },
    RegisterData {
        reg: 0x22,
        val: 0x1a,
    },
    RegisterData {
        reg: 0x37,
        val: 0xc3,
    },
    RegisterData {
        reg: 0x23,
        val: 0x00,
    },
    RegisterData {
        reg: 0x34,
        val: 0xc0,
    },
    RegisterData {
        reg: 0x36,
        val: 0x1a,
    },
    RegisterData {
        reg: 0x06,
        val: 0x88,
    },
    RegisterData {
        reg: 0x07,
        val: 0xc0,
    },
    RegisterData {
        reg: 0x0d,
        val: 0x87,
    },
    RegisterData {
        reg: 0x0e,
        val: 0x41,
    },
    RegisterData {
        reg: 0x4c,
        val: 0x00,
    },
    RegisterData {
        reg: 0xff,
        val: 0x00,
    },
    RegisterData {
        reg: 0xe0,
        val: 0x04,
    },
    RegisterData {
        reg: 0xc0,
        val: 0x64,
    },
    RegisterData {
        reg: 0xc1,
        val: 0x4b,
    },
    RegisterData {
        reg: 0x86,
        val: 0x35,
    },
    RegisterData {
        reg: 0x50,
        val: 0x89,
    },
    RegisterData {
        reg: 0x51,
        val: 0xc8,
    },
    RegisterData {
        reg: 0x52,
        val: 0x96,
    },
    RegisterData {
        reg: 0x53,
        val: 0x00,
    },
    RegisterData {
        reg: 0x54,
        val: 0x00,
    },
    RegisterData {
        reg: 0x55,
        val: 0x00,
    },
    RegisterData {
        reg: 0x57,
        val: 0x00,
    },
    RegisterData {
        reg: 0xe0,
        val: 0x00,
    },
    RegisterData {
        reg: 0xff,
        val: 0xff,
    },
];

pub const OV2640_64X64_ROI_FIXED: &[RegisterData] = &[
    RegisterData {
        reg: 0xff,
        val: 0x01,
    }, // Bank 1
    RegisterData {
        reg: 0x12,
        val: 0x00,
    }, // COM7: UXGA
    // Window Center 64x64
    RegisterData {
        reg: 0x17,
        val: 0x30,
    }, // HREFST
    RegisterData {
        reg: 0x18,
        val: 0x34,
    }, // HREFEND
    RegisterData {
        reg: 0x32,
        val: 0x00,
    }, // LSBs
    RegisterData {
        reg: 0x19,
        val: 0x47,
    }, // VSTRT
    RegisterData {
        reg: 0x1a,
        val: 0x4f,
    }, // VEND
    RegisterData {
        reg: 0x03,
        val: 0x00,
    }, // LSBs
    // Other settings (Bank 1)
    RegisterData {
        reg: 0x4f,
        val: 0xca,
    },
    RegisterData {
        reg: 0x50,
        val: 0xa8,
    },
    // --- Switch to Bank 0 ---
    RegisterData {
        reg: 0xff,
        val: 0x00,
    },
    // DSP Output Size (Move ZMOW here!)
    RegisterData {
        reg: 0x5a,
        val: 0x10,
    },
    RegisterData {
        reg: 0x5b,
        val: 0x10,
    },
    RegisterData {
        reg: 0x5c,
        val: 0x00,
    },
    // Other Settings from original
    RegisterData {
        reg: 0x6d,
        val: 0x00,
    },
    RegisterData {
        reg: 0x39,
        val: 0x12,
    },
    RegisterData {
        reg: 0x35,
        val: 0xda,
    },
    RegisterData {
        reg: 0x22,
        val: 0x1a,
    },
    RegisterData {
        reg: 0x37,
        val: 0xc3,
    },
    RegisterData {
        reg: 0x23,
        val: 0x00,
    },
    RegisterData {
        reg: 0x34,
        val: 0xc0,
    },
    RegisterData {
        reg: 0x36,
        val: 0x1a,
    },
    RegisterData {
        reg: 0x06,
        val: 0x88,
    },
    RegisterData {
        reg: 0x07,
        val: 0xc0,
    },
    RegisterData {
        reg: 0x0d,
        val: 0x87,
    },
    RegisterData {
        reg: 0x0e,
        val: 0x41,
    },
    RegisterData {
        reg: 0x4c,
        val: 0x00,
    },
    // Image/Window Size (Bank 0)
    RegisterData {
        reg: 0xe0,
        val: 0x04,
    },
    RegisterData {
        reg: 0xc0,
        val: 0x08,
    },
    RegisterData {
        reg: 0xc1,
        val: 0x08,
    },
    RegisterData {
        reg: 0x86,
        val: 0x35,
    },
    RegisterData {
        reg: 0x50,
        val: 0x89,
    },
    RegisterData {
        reg: 0x51,
        val: 0x10,
    },
    RegisterData {
        reg: 0x52,
        val: 0x10,
    },
    RegisterData {
        reg: 0xe0,
        val: 0x00,
    },
    RegisterData {
        reg: 0xff,
        val: 0xff,
    },
];

pub const OV2640_QVGA: &[RegisterData] = &[
    RegisterData {
        reg: 0xff,
        val: 0x0,
    },
    RegisterData {
        reg: 0x2c,
        val: 0xff,
    },
    RegisterData {
        reg: 0x2e,
        val: 0xdf,
    },
    RegisterData {
        reg: 0xff,
        val: 0x1,
    },
    RegisterData {
        reg: 0x3c,
        val: 0x32,
    },
    RegisterData {
        reg: 0x11,
        val: 0x0,
    },
    RegisterData { reg: 0x9, val: 0x2 },
    RegisterData {
        reg: 0x4,
        val: 0xa8,
    },
    RegisterData {
        reg: 0x13,
        val: 0xe5,
    },
    RegisterData {
        reg: 0x14,
        val: 0x48,
    },
    RegisterData {
        reg: 0x2c,
        val: 0xc,
    },
    RegisterData {
        reg: 0x33,
        val: 0x78,
    },
    RegisterData {
        reg: 0x3a,
        val: 0x33,
    },
    RegisterData {
        reg: 0x3b,
        val: 0xfb,
    },
    RegisterData {
        reg: 0x3e,
        val: 0x0,
    },
    RegisterData {
        reg: 0x43,
        val: 0x11,
    },
    RegisterData {
        reg: 0x16,
        val: 0x10,
    },
    RegisterData {
        reg: 0x39,
        val: 0x2,
    },
    RegisterData {
        reg: 0x35,
        val: 0x88,
    },
    RegisterData {
        reg: 0x22,
        val: 0xa,
    },
    RegisterData {
        reg: 0x37,
        val: 0x40,
    },
    RegisterData {
        reg: 0x23,
        val: 0x0,
    },
    RegisterData {
        reg: 0x34,
        val: 0xa0,
    },
    RegisterData { reg: 0x6, val: 0x2 },
    RegisterData {
        reg: 0x6,
        val: 0x88,
    },
    RegisterData {
        reg: 0x7,
        val: 0xc0,
    },
    RegisterData {
        reg: 0xd,
        val: 0xb7,
    },
    RegisterData { reg: 0xe, val: 0x1 },
    RegisterData {
        reg: 0x4c,
        val: 0x0,
    },
    RegisterData {
        reg: 0x4a,
        val: 0x81,
    },
    RegisterData {
        reg: 0x21,
        val: 0x99,
    },
    RegisterData {
        reg: 0x24,
        val: 0x40,
    },
    RegisterData {
        reg: 0x25,
        val: 0x38,
    },
    RegisterData {
        reg: 0x26,
        val: 0x82,
    },
    RegisterData {
        reg: 0x5c,
        val: 0x0,
    },
    RegisterData {
        reg: 0x63,
        val: 0x0,
    },
    RegisterData {
        reg: 0x46,
        val: 0x22,
    },
    RegisterData {
        reg: 0xc,
        val: 0x3a,
    },
    RegisterData {
        reg: 0x5d,
        val: 0x55,
    },
    RegisterData {
        reg: 0x5e,
        val: 0x7d,
    },
    RegisterData {
        reg: 0x5f,
        val: 0x7d,
    },
    RegisterData {
        reg: 0x60,
        val: 0x55,
    },
    RegisterData {
        reg: 0x61,
        val: 0x70,
    },
    RegisterData {
        reg: 0x62,
        val: 0x80,
    },
    RegisterData {
        reg: 0x7c,
        val: 0x5,
    },
    RegisterData {
        reg: 0x20,
        val: 0x80,
    },
    RegisterData {
        reg: 0x28,
        val: 0x30,
    },
    RegisterData {
        reg: 0x6c,
        val: 0x0,
    },
    RegisterData {
        reg: 0x6d,
        val: 0x80,
    },
    RegisterData {
        reg: 0x6e,
        val: 0x0,
    },
    RegisterData {
        reg: 0x70,
        val: 0x2,
    },
    RegisterData {
        reg: 0x71,
        val: 0x94,
    },
    RegisterData {
        reg: 0x73,
        val: 0xc1,
    },
    RegisterData {
        reg: 0x3d,
        val: 0x34,
    },
    RegisterData {
        reg: 0x12,
        val: 0x4,
    },
    RegisterData {
        reg: 0x5a,
        val: 0x57,
    },
    RegisterData {
        reg: 0x4f,
        val: 0xbb,
    },
    RegisterData {
        reg: 0x50,
        val: 0x9c,
    },
    RegisterData {
        reg: 0xff,
        val: 0x0,
    },
    RegisterData {
        reg: 0xe5,
        val: 0x7f,
    },
    RegisterData {
        reg: 0xf9,
        val: 0xc0,
    },
    RegisterData {
        reg: 0x41,
        val: 0x24,
    },
    RegisterData {
        reg: 0xe0,
        val: 0x14,
    },
    RegisterData {
        reg: 0x76,
        val: 0xff,
    },
    RegisterData {
        reg: 0x33,
        val: 0xa0,
    },
    RegisterData {
        reg: 0x42,
        val: 0x20,
    },
    RegisterData {
        reg: 0x43,
        val: 0x18,
    },
    RegisterData {
        reg: 0x4c,
        val: 0x0,
    },
    RegisterData {
        reg: 0x87,
        val: 0xd0,
    },
    RegisterData {
        reg: 0x88,
        val: 0x3f,
    },
    RegisterData {
        reg: 0xd7,
        val: 0x3,
    },
    RegisterData {
        reg: 0xd9,
        val: 0x10,
    },
    RegisterData {
        reg: 0xd3,
        val: 0x82,
    },
    RegisterData {
        reg: 0xc8,
        val: 0x8,
    },
    RegisterData {
        reg: 0xc9,
        val: 0x80,
    },
    RegisterData {
        reg: 0x7c,
        val: 0x0,
    },
    RegisterData {
        reg: 0x7d,
        val: 0x0,
    },
    RegisterData {
        reg: 0x7c,
        val: 0x3,
    },
    RegisterData {
        reg: 0x7d,
        val: 0x48,
    },
    RegisterData {
        reg: 0x7d,
        val: 0x48,
    },
    RegisterData {
        reg: 0x7c,
        val: 0x8,
    },
    RegisterData {
        reg: 0x7d,
        val: 0x20,
    },
    RegisterData {
        reg: 0x7d,
        val: 0x10,
    },
    RegisterData {
        reg: 0x7d,
        val: 0xe,
    },
    RegisterData {
        reg: 0x90,
        val: 0x0,
    },
    RegisterData {
        reg: 0x91,
        val: 0xe,
    },
    RegisterData {
        reg: 0x91,
        val: 0x1a,
    },
    RegisterData {
        reg: 0x91,
        val: 0x31,
    },
    RegisterData {
        reg: 0x91,
        val: 0x5a,
    },
    RegisterData {
        reg: 0x91,
        val: 0x69,
    },
    RegisterData {
        reg: 0x91,
        val: 0x75,
    },
    RegisterData {
        reg: 0x91,
        val: 0x7e,
    },
    RegisterData {
        reg: 0x91,
        val: 0x88,
    },
    RegisterData {
        reg: 0x91,
        val: 0x8f,
    },
    RegisterData {
        reg: 0x91,
        val: 0x96,
    },
    RegisterData {
        reg: 0x91,
        val: 0xa3,
    },
    RegisterData {
        reg: 0x91,
        val: 0xaf,
    },
    RegisterData {
        reg: 0x91,
        val: 0xc4,
    },
    RegisterData {
        reg: 0x91,
        val: 0xd7,
    },
    RegisterData {
        reg: 0x91,
        val: 0xe8,
    },
    RegisterData {
        reg: 0x91,
        val: 0x20,
    },
    RegisterData {
        reg: 0x92,
        val: 0x0,
    },
    RegisterData {
        reg: 0x93,
        val: 0x6,
    },
    RegisterData {
        reg: 0x93,
        val: 0xe3,
    },
    RegisterData {
        reg: 0x93,
        val: 0x3,
    },
    RegisterData {
        reg: 0x93,
        val: 0x3,
    },
    RegisterData {
        reg: 0x93,
        val: 0x0,
    },
    RegisterData {
        reg: 0x93,
        val: 0x2,
    },
    RegisterData {
        reg: 0x93,
        val: 0x0,
    },
    RegisterData {
        reg: 0x93,
        val: 0x0,
    },
    RegisterData {
        reg: 0x93,
        val: 0x0,
    },
    RegisterData {
        reg: 0x93,
        val: 0x0,
    },
    RegisterData {
        reg: 0x93,
        val: 0x0,
    },
    RegisterData {
        reg: 0x93,
        val: 0x0,
    },
    RegisterData {
        reg: 0x93,
        val: 0x0,
    },
    RegisterData {
        reg: 0x96,
        val: 0x0,
    },
    RegisterData {
        reg: 0x97,
        val: 0x8,
    },
    RegisterData {
        reg: 0x97,
        val: 0x19,
    },
    RegisterData {
        reg: 0x97,
        val: 0x2,
    },
    RegisterData {
        reg: 0x97,
        val: 0xc,
    },
    RegisterData {
        reg: 0x97,
        val: 0x24,
    },
    RegisterData {
        reg: 0x97,
        val: 0x30,
    },
    RegisterData {
        reg: 0x97,
        val: 0x28,
    },
    RegisterData {
        reg: 0x97,
        val: 0x26,
    },
    RegisterData {
        reg: 0x97,
        val: 0x2,
    },
    RegisterData {
        reg: 0x97,
        val: 0x98,
    },
    RegisterData {
        reg: 0x97,
        val: 0x80,
    },
    RegisterData {
        reg: 0x97,
        val: 0x0,
    },
    RegisterData {
        reg: 0x97,
        val: 0x0,
    },
    RegisterData {
        reg: 0xa4,
        val: 0x0,
    },
    RegisterData {
        reg: 0xa8,
        val: 0x0,
    },
    RegisterData {
        reg: 0xc5,
        val: 0x11,
    },
    RegisterData {
        reg: 0xc6,
        val: 0x51,
    },
    RegisterData {
        reg: 0xbf,
        val: 0x80,
    },
    RegisterData {
        reg: 0xc7,
        val: 0x10,
    },
    RegisterData {
        reg: 0xb6,
        val: 0x66,
    },
    RegisterData {
        reg: 0xb8,
        val: 0xa5,
    },
    RegisterData {
        reg: 0xb7,
        val: 0x64,
    },
    RegisterData {
        reg: 0xb9,
        val: 0x7c,
    },
    RegisterData {
        reg: 0xb3,
        val: 0xaf,
    },
    RegisterData {
        reg: 0xb4,
        val: 0x97,
    },
    RegisterData {
        reg: 0xb5,
        val: 0xff,
    },
    RegisterData {
        reg: 0xb0,
        val: 0xc5,
    },
    RegisterData {
        reg: 0xb1,
        val: 0x94,
    },
    RegisterData {
        reg: 0xb2,
        val: 0xf,
    },
    RegisterData {
        reg: 0xc4,
        val: 0x5c,
    },
    RegisterData {
        reg: 0xa6,
        val: 0x0,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x20,
    },
    RegisterData {
        reg: 0xa7,
        val: 0xd8,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x1b,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x31,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x0,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x18,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x20,
    },
    RegisterData {
        reg: 0xa7,
        val: 0xd8,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x19,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x31,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x0,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x18,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x20,
    },
    RegisterData {
        reg: 0xa7,
        val: 0xd8,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x19,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x31,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x0,
    },
    RegisterData {
        reg: 0xa7,
        val: 0x18,
    },
    RegisterData {
        reg: 0x7f,
        val: 0x0,
    },
    RegisterData {
        reg: 0xe5,
        val: 0x1f,
    },
    RegisterData {
        reg: 0xe1,
        val: 0x77,
    },
    RegisterData {
        reg: 0xdd,
        val: 0x7f,
    },
    RegisterData {
        reg: 0xc2,
        val: 0xe,
    },
    RegisterData {
        reg: 0xff,
        val: 0x0,
    },
    RegisterData {
        reg: 0xe0,
        val: 0x4,
    },
    RegisterData {
        reg: 0xc0,
        val: 0xc8,
    },
    RegisterData {
        reg: 0xc1,
        val: 0x96,
    },
    RegisterData {
        reg: 0x86,
        val: 0x3d,
    },
    RegisterData {
        reg: 0x51,
        val: 0x90,
    },
    RegisterData {
        reg: 0x52,
        val: 0x2c,
    },
    RegisterData {
        reg: 0x53,
        val: 0x0,
    },
    RegisterData {
        reg: 0x54,
        val: 0x0,
    },
    RegisterData {
        reg: 0x55,
        val: 0x88,
    },
    RegisterData {
        reg: 0x57,
        val: 0x0,
    },
    RegisterData {
        reg: 0x50,
        val: 0x92,
    },
    RegisterData {
        reg: 0x5a,
        val: 0x50,
    },
    RegisterData {
        reg: 0x5b,
        val: 0x3c,
    },
    RegisterData {
        reg: 0x5c,
        val: 0x0,
    },
    RegisterData {
        reg: 0xd3,
        val: 0x4,
    },
    RegisterData {
        reg: 0xe0,
        val: 0x0,
    },
    RegisterData {
        reg: 0xff,
        val: 0x0,
    },
    RegisterData { reg: 0x5, val: 0x0 },
    RegisterData {
        reg: 0xda,
        val: 0x9,
    },
    RegisterData {
        reg: 0xd7,
        val: 0x3,
    },
    RegisterData {
        reg: 0xe0,
        val: 0x0,
    },
    RegisterData { reg: 0x5, val: 0x0 },
    RegisterData {
        reg: 0xff,
        val: 0xff,
    },
];

pub const OV2640_64X64_NATIVE: &[RegisterData] = &[
    RegisterData {
        reg: 0xff,
        val: 0x01,
    }, // Bank 1
    RegisterData {
        reg: 0x12,
        val: 0x02,
    }, // COM7: UXGA + Color Bar Test Pattern
    // Window Center 64x64 (Same as ROI_FIXED)
    RegisterData {
        reg: 0x17,
        val: 0x30,
    }, // HREFST
    RegisterData {
        reg: 0x18,
        val: 0x34,
    }, // HREFEND
    RegisterData {
        reg: 0x32,
        val: 0x00,
    }, // LSBs
    RegisterData {
        reg: 0x19,
        val: 0x47,
    }, // VSTRT
    RegisterData {
        reg: 0x1a,
        val: 0x4f,
    }, // VEND
    RegisterData {
        reg: 0x03,
        val: 0x00,
    }, // LSBs
    // Other settings (Bank 1) - from ROI_FIXED
    RegisterData {
        reg: 0x4f,
        val: 0xca,
    },
    RegisterData {
        reg: 0x50,
        val: 0xa8,
    },
    // --- Switch to Bank 0 ---
    RegisterData {
        reg: 0xff,
        val: 0x00,
    },
    // DSP Output Size (64x64) - ZMOW/ZMOH
    RegisterData {
        reg: 0x5a,
        val: 0x10,
    },
    RegisterData {
        reg: 0x5b,
        val: 0x10,
    },
    RegisterData {
        reg: 0x5c,
        val: 0x00,
    },
    // OUT_SIZE (64x64)
    RegisterData {
        reg: 0x51,
        val: 0x10,
    },
    RegisterData {
        reg: 0x52,
        val: 0x10,
    },
    // RGB565 and format settings
    RegisterData {
        reg: 0xda,
        val: 0x09,
    }, // RGB565 Output
    // Image/Window Size (Bank 0)
    RegisterData {
        reg: 0xe0,
        val: 0x04,
    },
    RegisterData {
        reg: 0xc0,
        val: 0x08,
    }, // HSIZE 64
    RegisterData {
        reg: 0xc1,
        val: 0x08,
    }, // VSIZE 64
    RegisterData {
        reg: 0xe0,
        val: 0x00,
    },
    RegisterData {
        reg: 0xff,
        val: 0xff,
    },
];
