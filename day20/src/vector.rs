use std::ops::{Add, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Vec2(pub i64, pub i64);

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Vec2 {
    pub fn neighbors(self) -> Vec<Vec2> {
        vec![
            self + Vec2(1, 0),
            self + Vec2(-1, 0),
            self + Vec2(0, 1),
            self + Vec2(0, -1),
        ]
    }
}
