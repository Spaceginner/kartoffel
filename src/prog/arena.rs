use alloc::format;
use kartoffel::{arm_stab, arm_wait, serial_write};
use rand::prelude::*;
use crate::compass::get_dir;
use crate::dir::RelDir;
use crate::motor;
use crate::radar::Radar;

#[derive(Debug, Copy, Clone)]
enum State {
    Safe,
    Danger(u8),
    Attack(i8, i8),
    Stab(RelDir),
}


fn distance_to(x: i8, y: i8) -> u8 {
    x.unsigned_abs() + y.unsigned_abs()
}


fn decide_state(cur: Option<State>, radar: &Radar) -> State {
    radar.print_out_aligned();

    let mut found = None;
    let d = radar.radius() as i8;
    for y in -d..=d {
        for x in -d..=d {
            if x == 0 && y == 0 {
                continue;
            };
            
            if radar.at(x, y) == '@' {
                serial_write(format!("spot @ {x}/{y}\n"));
                
                if let Some((_, dist)) = found {
                    let new_dist = distance_to(x, y);
                    if new_dist < dist {
                        found = Some(((x, y), new_dist));
                    };
                } else {
                    found = Some(((x, y), distance_to(x, y)));
                };
            };
        };
    };

    if let Some((coord, dist)) = found {
        serial_write(format!("chose @ {}/{}\n", coord.0, coord.1));
        
        match dist {
            0 => unreachable!(),
            1 => {
                State::Stab(match coord {
                    (0, -1) => RelDir::Forward,
                    (0, 1) => RelDir::Backward,
                    (-1, 0) => RelDir::Leftward,
                    (1, 0) => RelDir::Rightward,
                    _ => unreachable!(),
                })
            },
            2..5 => { State::Attack(coord.0, coord.1) },
            5.. => { State::Danger(5) },
        }
    } else if let Some(State::Danger(dngr)) = cur {
        if dngr != 0 {
            State::Danger(dngr - 1)
        } else {
            State::Safe
        }
    } else if let Some(State::Attack(..)) = cur {
        State::Danger(0)
    } else {
        State::Safe
    }
}


pub fn main() {
    let mut state = None;
    let mut rng = rand_xoshiro::Xoroshiro64Star::seed_from_u64(get_dir() as u64);

    let mut radar = Radar::new(4);
    loop {
        state = Some(decide_state(state, &radar));
        
        match state {
            None | Some(State::Safe) => {
                serial_write("safe\n");
                
                // todo check if we can actually go there
                motor::turn_by(*RelDir::CLOCKWISE.choose(&mut rng).unwrap());
                motor::forward();
                
                radar.update(4);
            },
            Some(State::Danger(count)) => {
                serial_write(format!("danger {count}\n"));

                radar.update(3);
            },
            Some(State::Attack(x, y)) => {
                serial_write("attack\n");
                
                // it took an A5 page to do the math for this one lol
                if x == 0 {
                    if y < 0 {
                        motor::forward();
                    } else {
                        motor::left();  // honestly we could decide randomly lol (or towards where there are more enemies?)
                    }
                } else if y >= 0 || y.abs() > x.abs() {
                    if x < 0 {
                        motor::left();
                    } else {
                        motor::right();
                    }
                } else {
                    motor::forward();
                };
                
                radar.update(2);
            },
            Some(State::Stab(rel)) => {
                serial_write("stab\n");

                motor::turn_by(rel);
                arm_wait();
                arm_stab();
                
                radar.update(2);
            }
        }
    }
}
