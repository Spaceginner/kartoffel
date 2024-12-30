mod navigation;

use alloc::format;
use alloc::vec::Vec;
use kartoffel::{arm_pick, radar_scan_3x3, radar_wait, RadarScan, serial_write};
use crate::dir::Dir;
use crate::compass;
use crate::motor;
use navigation::{Map, Pos};


#[derive(Clone, Debug)]
struct ExhaustedDirs([bool; 4]);

impl ExhaustedDirs {
    pub fn new<const R: usize>(radar: &RadarScan<R>, cur_pos: Pos, map: &Map) -> Self {
        let cur_dir = compass::get_dir();
        
        let mut dirs = [false; 4];
        
        for dir in Dir::CLOCKWISE {
            let (dx, dy) = dir.rel(cur_dir).offset();
            dirs[dir as usize] = radar.at(dx, dy) != '.' || map.visited(cur_pos + dir.offset());
        };
        
        Self(dirs)
    }
    
    pub fn exhaust(&mut self, dir: Dir) {
        self.0[dir as usize] = true;
    }
    
    pub fn is_exhausted(&self, dir: Dir) -> bool {
        self.0[dir as usize]
    }

    pub fn find_non_exhausted(&self, dirs: &[Dir]) -> Option<Dir> {
        dirs.iter().find(|dir| !self.is_exhausted(**dir)).copied()
    }

    pub fn find_first_non_exhausted(&self) -> Option<Dir> {
        self.find_non_exhausted(&Dir::CLOCKWISE)
    }
    
    pub fn find_prioritized(&self, goals: &[Dir]) -> Option<Dir> {
        self.find_non_exhausted(goals).or_else(|| self.find_first_non_exhausted())
    }
}


pub fn check_for_goal<const R: usize>(radar: &RadarScan<R>) -> bool {
    serial_write("checking for goal\n");
    if radar.at(0, -1) == '=' {
        arm_pick();
        true
    } else if radar.at(-1, 0) == '=' {
        motor::left();
        arm_pick();
        true
    } else if radar.at(1, 0) == '=' {
        motor::right();
        arm_pick();
        true
    } else {
        false
    }
}


pub fn main() {
    // todo maybe adjust the goal more accordingly? eg if we found a specific flag, tho not sure how to do detect what flag have we found
    for goal_ver in [Dir::North, Dir::South] {
        for goal_hor in [Dir::West, Dir::East] {
            serial_write(format!("goal: {goal_ver:?}/{goal_hor:?}\n"));

            radar_wait();
            let mut radar = radar_scan_3x3();

            let mut map = Map::default();
            let mut cur_pos = Pos::default();
            map.visit(cur_pos);
            let mut prev_inter_infos = Vec::new();

            fn build_inter_info<const R: usize>(radar: &RadarScan<R>, cur_pos: Pos, map: &Map) -> (ExhaustedDirs, Dir) {
                (ExhaustedDirs::new(radar, cur_pos, map), compass::get_dir().opposite())
            }

            let mut inter_info = build_inter_info(&radar, cur_pos, &map);
            
            loop {
                radar_wait();

                if check_for_goal(&radar) {
                    serial_write("!!!: goal\n");
                    break;
                };

                // todo first branch to undiscovered goal sides
                if let Some(dir) = inter_info.0.find_prioritized(&[goal_hor, goal_ver]) {
                    serial_write(format!("new: {dir:?}\n"));

                    inter_info.0.exhaust(dir);
                    
                    motor::turn_to(dir);
                    motor::forward();

                    radar_wait();
                    radar = radar_scan_3x3();
                    
                    cur_pos += dir.offset();
                    map.visit(cur_pos);
                    
                    prev_inter_infos.push(inter_info);
                    inter_info = build_inter_info(&radar, cur_pos, &map);
                } else {
                    serial_write(format!("exh: {:?}\n", inter_info.1));
                    
                    motor::turn_to(inter_info.1);
                    motor::forward();

                    cur_pos += inter_info.1.offset();

                    radar_wait();
                    radar = radar_scan_3x3();
                    
                    inter_info = prev_inter_infos.pop().unwrap_or_else(|| panic!("no way"));
                };
            };
        };
    };
}
