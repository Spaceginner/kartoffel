use hashbrown::HashSet;

#[derive(Debug, Clone, Default)]
pub struct Map(HashSet<Pos>);


impl Map {
    pub fn visit(&mut self, pos: Pos) {
        self.0.insert(pos);
    }

    pub fn visited(&self, pos: Pos) -> bool {
        self.0.contains(&pos)
    }
}


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i16,
    pub y: i16,
}

impl core::ops::Add<(i16, i16)> for Pos {
    type Output = Self;

    fn add(self, rhs: (i16, i16)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl core::ops::AddAssign<(i16, i16)> for Pos {
    fn add_assign(&mut self, rhs: (i16, i16)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}
