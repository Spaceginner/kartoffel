use core::num::NonZeroU64;
use kartoffel::serial_write;
use crate::dir::Dir;
use crate::{compass, sysk};


// safety is utmost importance!
pub static mut IS_VALID: bool = true;
pub static mut ALREADY_EXISTS: bool = false; 


pub fn require_update() {
    unsafe { IS_VALID = false };
}


#[derive(Debug)]
pub struct Radar {
    radius: usize,
}


impl Drop for Radar {
    fn drop(&mut self) {
        unsafe { ALREADY_EXISTS = false };
    }
}


impl Radar {
    pub fn new(radius: usize) -> Self {
        if !matches!(radius, 0..=4) {
            panic!("invalid radius");
        };
        
        if unsafe { ALREADY_EXISTS } {
            panic!("another radar already exists");
        } else {
            unsafe { ALREADY_EXISTS = true };
        }
        
        let radar = Self { radius };
        
        if radius != 0 {
            kartoffel::radar_wait();
            unsafe { sysk::write(sysk::MEM_RADAR, 0, radar.diameter() as u32); }
        };
        
        unsafe { IS_VALID = true };
        
        radar
    }
    
    pub fn update(&mut self, radius: usize) {
        if !matches!(radius, 1..=4) {
            panic!("invalid radius");
        };

        kartoffel::radar_wait();
        unsafe { sysk::write(sysk::MEM_RADAR, 0, self.diameter() as u32); }

        unsafe { IS_VALID = true };
        
        self.radius = radius;
    }
    
    pub fn radius(&self) -> usize {
        self.radius
    }
    
    pub fn diameter(&self) -> usize {
        self.radius * 2 + 1
    }
    
    pub fn at(&self, dx: i8, dy: i8) -> char {
        if !unsafe { IS_VALID } {
            panic!("used invalid radar");
        };
        
        let c = self.slice(dx, dy, 0) as u8 as char;
        
        if !['=', '.', '@', '-', '|', '=', ' '].contains(&c) {
            panic!("radar broke ({:0>2x} at {dx}/{dy}, r{}, of{})", c as u8, self.radius, self.comp_offset(dx, dy, 0));
        };
        
        c
    }

    pub fn at_abs(&self, dx: i8, dy: i8) -> char {
        match compass::get_dir() {
            Dir::North => self.at(dx, dy),
            Dir::East => self.at(dy, -dx),
            Dir::South => self.at(-dx, -dy),
            Dir::West => self.at(-dy, dx),
        }
    }
    
    pub fn bot_at(&self, dx: i8, dy: i8) -> Option<NonZeroU64> {
        if !unsafe { IS_VALID } {
            panic!("used invalid radar");
        };
        
        let id = (
            self.slice(dx, dy, 1) as u64,
            self.slice(dx, dy, 2) as u64,
        );
        
        NonZeroU64::new((id.0 << 32) | id.1)
    }

    pub fn bot_at_abs(&self, dx: i8, dy: i8) -> Option<NonZeroU64> {
        match compass::get_dir() {
            Dir::North => self.bot_at(dx, dy),
            Dir::East => self.bot_at(dy, -dx),
            Dir::South => self.bot_at(-dx, -dy),
            Dir::West => self.bot_at(-dy, dx),
        }
    }

    pub fn print_out_aligned(&self) {
        serial_write("/radar A/\n");
        let d = self.radius() as i8;
        for y in -d..=d {
            for x in -d..=d {
                serial_write(self.at_abs(x, y));
            };
            serial_write("\n");
        }
        serial_write("\\radar\\\n");
    }

    pub fn print_out(&self) {
        serial_write("/radar/\n");
        let d = self.radius() as i8;
        for y in -d..=d {
            for x in -d..=d {
                serial_write(self.at(x, y));
            };
            serial_write("\n");
        };
        serial_write("\\radar\\\n");
    }
    
    fn comp_offset(&self, dx: i8, dy: i8, z: usize) -> usize {
        let x = (dx + self.radius as i8) as usize;
        let y = (dy + self.radius as i8) as usize;
        let d = self.diameter();
        
        1 + z * d * d + y * d + x
    }
    
    fn slice(&self, dx: i8, dy: i8, z: usize) -> u32 {
        if ![0, 1, 2].contains(&z) {
            panic!("invalid z");
        };
        if dx.abs() > self.radius as i8 {
            panic!("dx out of bounds {dx}>{}", self.radius);
        };
        if dy.abs() > self.radius as i8 {
            panic!("dy out of bounds {dy}>{}", self.radius);
        };
        
        unsafe {
            sysk::read(
                sysk::MEM_RADAR,
                self.comp_offset(dx, dy, z)
            )
        }
    }
}
