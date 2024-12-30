use kartoffel::*;
use crate::motor;

pub fn main() {
    motor::move_forward(9);

    let mut safe = false;
    while !safe {
        radar_wait();
        let radar = radar_scan_7x7();
        safe =
            radar.at(1, -3) != '@'
            && radar.at(1, -2) != '@'
            && radar.at(0, -2) != '@';
    };

    motor::move_forward(2);

    motor::left();

    motor::move_forward(3);

    motor::right();

    motor::move_forward(3);

    arm_pick();

    motor::move_forward(1);

    motor::left();

    motor::move_forward(2);

    let mut safe = false;
    while !safe {
        radar_wait();
        let radar = radar_scan_5x5();
        safe =
            radar.at(-2, -2) != '@'
                && radar.at(-1, -2) != '@'
                && radar.at(0, -2) != '@';
    };

    motor::move_forward(1);

    motor::right();

    loop {
        motor_step();
        motor_wait();
    };
}
