use kartoffel::*;

pub fn main() {
    loop {
        radar_wait();
        let radar = radar_scan_3x3();

        if radar.at(0, -1) == '@' {
            arm_stab();
        };

        if !['|', '-'].contains(&radar.at(-1, 0)) {
            serial_write("no wall left\n");
            motor_turn_left();
            motor_wait();
            motor_step();
            motor_wait();
        } else if ['|', '-'].contains(&radar.at(0, -1)) {
            serial_write("wall forward\n");
            motor_turn_right();
            motor_wait();
        } else {
            serial_write("no wall forward\n");
            motor_step();
            motor_wait();
        };
    };
}
