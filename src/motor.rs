use alloc::format;
use kartoffel::{motor_step, motor_turn_left, motor_turn_right, motor_wait, serial_write};
use crate::radar;
use crate::compass;
use crate::dir::{Dir, RelDir};


pub fn forward() {
    motor_wait();
    motor_step();
    radar::require_update();
}

pub fn right() {
    motor_wait();
    motor_turn_right();
    compass::patch(RelDir::Rightward);
    radar::require_update();
}

pub fn left() {
    motor_wait();
    motor_turn_left();
    compass::patch(RelDir::Leftward);
    radar::require_update();
}

pub fn around() {
    motor_wait();
    motor_turn_right();
    motor_wait();
    motor_turn_right();
    compass::patch(RelDir::Backward);
    radar::require_update();
}


pub fn turn_by(rel: RelDir) {
    match rel {
        RelDir::Forward => {},
        RelDir::Rightward => {
            right()
        },
        RelDir::Leftward => {
            left()
        },
        RelDir::Backward => {
            around()
        },
    };
}

pub fn turn_to(dir: Dir) {
    let cur_dir = compass::get_dir();
    let rel = dir.rel(cur_dir); 
    
    serial_write(format!("trn: {cur_dir:?} {rel:?} {dir:?}\n"));

    turn_by(rel);
}


pub fn move_forward(d: u32) {
    for _ in 0..d {
        forward()
    }
}
