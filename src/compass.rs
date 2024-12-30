use kartoffel::{compass_dir, serial_write};
use super::dir::{Dir, RelDir};

static mut DIR: Option<Dir> = None;


pub fn reset() -> Dir {
    serial_write("resetting compass\n");
    
    let mut dir_raw = 0;

    while dir_raw == 0 {
        dir_raw = compass_dir();
    };

    let dir = match dir_raw {
        1 => Dir::North,
        2 => Dir::East,
        3 => Dir::South,
        4 => Dir::West,
        _ => unreachable!()
    };
    
    unsafe {
        DIR = Some(dir);
    };
    
    dir
}


pub fn patch(rel_dir: RelDir) -> Dir {
    if let Some(dir) = unsafe { DIR } {
        match rel_dir {
            RelDir::Forward => dir,
            turn @ (RelDir::Leftward | RelDir::Rightward) => {
                let new_dir = dir.turn(turn);
                unsafe { DIR = Some(new_dir) };
                new_dir
            },
            RelDir::Backward => {
                let new_dir = dir.opposite();
                unsafe { DIR = Some(new_dir) };
                new_dir
            }
        }
    } else {
        reset()
    }
}


pub fn get_dir() -> Dir {
    unsafe { DIR }.unwrap_or_else(reset)
}
