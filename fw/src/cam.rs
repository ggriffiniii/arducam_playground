pub enum Subsampling {
    None = 1,    // UXGA
    Half = 2,    // SVGA
    Quarter = 4, // CIF
}

pub struct Camera;

impl Camera {
    pub fn write_reg(&self, _reg: u8, _val: u8) {}
    pub fn read_reg(&self, _reg: u8) -> u8 { 0 }

    fn set_horizontal_window(&self, start: u16, end: u16) {
        let hs = start / 2;
        let he = end / 2;
        self.write_reg(0x17, (hs >> 3) as u8); 
        self.write_reg(0x18, (he >> 3) as u8); 
        let mut reg32 = self.read_reg(0x32);
        reg32 = (reg32 & 0xC0) | (((he & 0x07) << 3) as u8) | ((hs & 0x07) as u8);
        self.write_reg(0x32, reg32);
        let mut arcom2 = self.read_reg(0x34);
        arcom2 = (arcom2 & !0x04) | if (hs & 0x01) != 0 { 0x04 } else { 0x00 };
        self.write_reg(0x34, arcom2);
    }

    fn set_vertical_window(&self, start: u16, end: u16) {
        let vs = start / 2;
        let ve = end / 2;
        self.write_reg(0x19, (vs >> 2) as u8); 
        self.write_reg(0x1a, (ve >> 2) as u8); 
        let mut com1 = self.read_reg(0x03);
        com1 = (com1 & 0xF0) | (((ve & 0x03) << 2) as u8) | ((vs & 0x03) as u8);
        self.write_reg(0x03, com1);
        let mut com19 = self.read_reg(0x48);
        com19 = (com19 & 0xFC) | ((vs & 0x03) as u8);
        self.write_reg(0x48, com19);
    }

    fn set_dsp_input_size(&self, width: u16, height: u16) {
        self.write_reg(0xc0, (width / 8) as u8);
        self.write_reg(0xc1, (height / 8) as u8);
    }

    fn set_dsp_output_size(&self, width: u16, height: u16) {
        let w4 = (width / 4) as u8;
        let h4 = (height / 4) as u8;
        self.write_reg(0x51, w4); 
        self.write_reg(0x52, h4); 
        self.write_reg(0x5a, w4); 
        self.write_reg(0x5b, h4); 

        let mut vhyx = self.read_reg(0x55);
        vhyx &= 0x77; 
        if width >= 1024 { vhyx |= 0x80; } 
        if height >= 1024 { vhyx |= 0x08; } 
        self.write_reg(0x55, vhyx);

        let mut zmhh = self.read_reg(0x5c);
        zmhh &= 0xF0; 
        if width >= 1024 { zmhh |= 0x01; } 
        if height >= 1024 { zmhh |= 0x04; } 
        self.write_reg(0x5c, zmhh);
    }

    pub fn builder() -> CameraBuilder {
        CameraBuilder::default()
    }
}

pub struct CameraBuilder {
    win_w: u16,
    win_h: u16,
    sampling: Subsampling,
    out_w: u16,
    out_h: u16,
    test_pattern: bool,
    clk_div: u8, // 0 to 63 [cite: source 960]
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            win_w: 1600,
            win_h: 1200,
            sampling: Subsampling::None,
            out_w: 1600,
            out_h: 1200,
            test_pattern: false,
            clk_div: 0, // Default to no division (fastest) [cite: source 960]
        }
    }
}

impl CameraBuilder {
    pub fn window(mut self, w: u16, h: u16) -> Self { self.win_w = w; self.win_h = h; self }
    pub fn subsampling(mut self, s: Subsampling) -> Self { self.sampling = s; self }
    pub fn output(mut self, w: u16, h: u16) -> Self { self.out_w = w; self.out_h = h; self }
    pub fn test_pattern(mut self, enabled: bool) -> Self { self.test_pattern = enabled; self }

    /// Sets the internal clock divider. [cite: source 960]
    /// Formula: Internal Clock = XVCLK / (value + 1)
    pub fn clock_divider(mut self, value: u8) -> Self {
        self.clk_div = value & 0x3F; // Only 6 bits available [cite: source 960]
        self
    }

    pub fn build(self, cam: &Camera) -> Result<(), &'static str> {
        if (self.win_w as u32 * self.out_h as u32) != (self.out_w as u32 * self.win_h as u32) {
            return Err("Aspect ratio mismatch");
        }

        match self.sampling {
            Subsampling::None => {
                if self.win_w % 2 != 0 || self.win_h % 4 != 0 { return Err("UXGA ROI requires 2x4 multiple"); }
            }
            _ => {
                if self.win_w % 2 != 0 || self.win_h % 2 != 0 { return Err("Subsampling ROI requires 2x2 multiple"); }
            }
        }

        // --- Bank 1: Sensor Core ---
        cam.write_reg(0xff, 0x01); 

        // Set Clock Divider (CLKRC) [cite: source 960]
        let mut clkrc = cam.read_reg(0x11);
        clkrc = (clkrc & 0xC0) | self.clk_div; // Preserve bits 7 (reserved) and 6 (clock option)
        cam.write_reg(0x11, clkrc);

        let mut com7 = cam.read_reg(0x12);
        com7 = (com7 & 0x8D) | match self.sampling {
            Subsampling::None => 0x00, Subsampling::Half => 0x40, Subsampling::Quarter => 0x10,
        } | if self.test_pattern { 0x02 } else { 0x00 };
        cam.write_reg(0x12, com7); [cite: source 969]

        let h_start = 816 - (self.win_w / 2);
        let v_start = 616 - (self.win_h / 2);
        cam.set_horizontal_window(h_start, h_start + self.win_w);
        cam.set_vertical_window(v_start, v_start + self.win_h);

        // --- Bank 0: DSP ---
        cam.write_reg(0xff, 0x00); 
        let factor = self.sampling as u16;
        cam.set_dsp_input_size(self.win_w / factor, self.win_h / factor);
        cam.set_dsp_output_size(self.out_w, self.out_h);

        Ok(())
    }
}