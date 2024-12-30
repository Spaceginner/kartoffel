use alloc::format;
use kartoffel::{motor_wait, radar_scan_7x7, radar_wait, serial_write};
use rand::prelude::*;
use crate::{compass, motor};
use crate::dir::{Dir, RelDir};
use crate::radar::Radar;
use crate::utils::sleep;


pub fn main() {
    let mut rng = rand_xoshiro::Xoroshiro64Star::seed_from_u64(compass::get_dir() as u64);
    let mut radar = Radar::new(3);
    
    loop {
        // todo check if we can actually go there
        motor::turn_by(*RelDir::CLOCKWISE.choose(&mut rng).unwrap());
        motor::forward();

        // radar_wait();
        // radar = radar_scan_7x7();
        radar.update(3);
        
        let dir = compass::get_dir();
        serial_write(format!("\n/// (adj {dir:?})\n"));
        // let d = 3;
        // for y in -d..=d {
        //     for x in -d..=d {
        //         serial_write(match dir {
        //             Dir::North => radar.at(x, y),
        //             Dir::East => radar.at(y, -x),
        //             Dir::South => radar.at(-x, -y),
        //             Dir::West => radar.at(-y, x),
        //         });
        //     };
        //     serial_write("\n");
        // };
        radar.print_out_aligned();
        
        sleep(30_000);
    }
}
