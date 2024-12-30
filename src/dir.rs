#[derive(Clone, Copy, Debug)]
pub enum Dir {
    North = 0, East = 1, South, West
}

impl Dir {
    pub const CLOCKWISE: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];
    
    pub fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    pub fn rel(self, to: Self) -> RelDir {
        let rel = self as i32 - to as i32;

        match rel {
            0 => RelDir::Forward,
            1 | -3 => RelDir::Rightward,
            2 | -2 => RelDir::Backward,
            3 | -1 => RelDir::Leftward,
            _ => unreachable!("rel: {} - {} = {rel}", self as i32, to as i32)
        }
    }

    pub fn turn(self, degree: RelDir) -> Self {
        let dir = self as i32 + degree as i32;

        match dir {
            0 | 4 => Self::North,
            1 | 5 => Self::East,
            2 => Self::South,
            3 | -1 => Self::West,
            _ => unreachable!("dir: {} + {} = {dir}", self as i32, degree as i32)
        }
    }

    pub fn offset(self) -> (i16, i16) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RelDir {
    Forward = 0, Leftward = -1, Rightward = 1, Backward = 2
}

impl RelDir {
    pub const CLOCKWISE: [Self; 4] = [Self::Forward, Self::Rightward, Self::Backward, Self::Leftward];
    
    pub fn offset(self) -> (i8, i8) {
        match self {
            Self::Forward => (0, -1),
            Self::Rightward => (1, 0),
            Self::Backward => (0, 1),
            Self::Leftward => (-1, 0),
        }
    }
}
