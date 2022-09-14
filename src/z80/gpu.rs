use crate::z80::cpu::CPU;
use crate::z80::memory::MMU;
use crate::z80::register::{ Reg8, Reg16 };

#[repr(u8)]
#[derive(Clone, Debug)]
enum GPUMode {
    HBlank       = 0,
    VBlank       = 1,
    /// OAM search -> compute which sprite are visible
    ScanlineOAM  = 2,
    /// Pixel transfer -> transfer pixel from the VRAM to the GPU?
    ScanlineVRAM = 3,
}

impl Default for GPUMode {
    fn default() -> GPUMode { GPUMode::HBlank }
}

impl GPUMode {
    pub fn cycle_length(&self) -> Reg16 {
        match self {
            GPUMode::ScanlineVRAM => 172,
            GPUMode::ScanlineOAM  => 80,
            GPUMode::VBlank       => 204,
            GPUMode::HBlank       => 456, // * 10 lines = 4560
        }
    }
}

#[repr(u8)]
#[derive(Clone, Debug)]
enum Colour {
    White = 0,
    LightGrey = 1,
    DarkGrey = 2,
    Black = 3,
}

struct Tile {}

#[derive(Default, Clone)]
pub struct GPU {
    /// GPU mode: is used to determine which operation to do
    mode: GPUMode,
    /// Clock used to determine when to change mode
    modeclock: Reg16,
    /// Which line we are working on
    line: u8,
    /// Flag used to know which tilemap we are using
    bgmap: bool,
}
/**
VRAM layout:
8000 - 87FF: Tileset #1: tiles 0   -> 127
8800 - 8FFF: Tileset #1: tiles 128 -> 255
             Tileset #0: tiles -1  -> -128
9000 - 97FF: Tileset #0: tiles 0   -> 127

9800 - 9BFF: Tile map #1
9C00 - 9FFF: Tile map #2

The tileset #1 is singed
The tileset #2 is unsigned
*/

impl GPU {
    pub fn step(&self, cpu: &CPU, memory: &MMU) -> GPU {
        let mut gpu = self.clone();
        println!("GPU: modeclock = {}; mode = {:?}; line = {}", self.modeclock, self.mode, self.line);
        gpu.modeclock += cpu.registers.clock.t;
        if gpu.modeclock >= gpu.mode.cycle_length() {
            match gpu.mode {
                GPUMode::ScanlineOAM => {
                    // If we spend enough time in this mode
                    // Since we're not doing a 1:1 emulation, this step isn't
                    // emulated so when the GPU is in it's mode, the emulated GPU
                    // is doing nothing for the 80 ticks it's in this mode
                    gpu.modeclock = 0;
                    gpu.mode = GPUMode::ScanlineVRAM;
                }
                GPUMode::ScanlineVRAM => {
                    // The end of this mode is treated as the end of the scanline
                    gpu.mode = GPUMode::HBlank;
                    gpu.modeclock = 0;
                    gpu.renderscan(&memory);
                }
                GPUMode::HBlank => {
                    gpu.modeclock = 0;
                    gpu.line += 1;
                    if gpu.line == 143 {
                        gpu.mode = GPUMode::VBlank;
                        gpu.draw();
                    } else {
                        gpu.mode = GPUMode::ScanlineOAM;
                    }
                }
                GPUMode::VBlank => {
                    gpu.modeclock = 0;
                    gpu.line += 1;
                    if gpu.line > 153 {
                        gpu.mode = GPUMode::ScanlineOAM;
                        gpu.line = 0;
                    }
                }
            }
        }
        gpu
    }

    fn make_tile(&self, memory: &MMU) -> Tile {
        unimplemented!()
    }

    /// Write a scanline to the framebuffer
    fn renderscan(&self, memory: &MMU) {
        unimplemented!()
    }
    fn draw(&self) {
        unimplemented!()
    }
}
