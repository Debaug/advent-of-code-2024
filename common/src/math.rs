use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vect {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn in_bounds(self, width: usize, height: usize) -> bool {
        usize::try_from(self.x).ok().is_some_and(|x| x < width)
            && usize::try_from(self.y).ok().is_some_and(|y| y < height)
    }
}

impl Vect {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add<Vect> for Pos {
    type Output = Pos;
    fn add(self, rhs: Vect) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vect> for Pos {
    type Output = Pos;
    fn sub(self, rhs: Vect) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Pos> for Pos {
    type Output = Vect;
    fn sub(self, rhs: Pos) -> Self::Output {
        Vect {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Vect> for isize {
    type Output = Vect;
    fn mul(self, rhs: Vect) -> Self::Output {
        Vect {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Neg for Vect {
    type Output = Vect;
    fn neg(self) -> Self::Output {
        Vect {
            x: -self.x,
            y: -self.y,
        }
    }
}
