#![allow(dead_code)]

use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Bound, RangeBounds, Sub, SubAssign};

#[derive(Copy, Clone, Default, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn wrapping_add(self, rhs: Self, range: impl RangeBounds<isize>) -> Self {
        let lower_bound = match range.start_bound() {
            Bound::Unbounded => isize::MIN,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        };
        let upper_bound = match range.end_bound() {
            Bound::Unbounded => isize::MAX,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x,
        };
        let x =
            lower_bound + (self.x - lower_bound.wrapping_add(rhs.x)) % (upper_bound - lower_bound);
        let y =
            lower_bound + (self.y - lower_bound.wrapping_add(rhs.y)) % (upper_bound - lower_bound);
        Self { x, y }
    }

    pub fn wrapping_sub(self, rhs: Self, range: impl RangeBounds<isize>) -> Self {
        let lower_bound = match range.start_bound() {
            Bound::Unbounded => isize::MIN,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        };
        let upper_bound = match range.end_bound() {
            Bound::Unbounded => isize::MAX,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x,
        };
        let x =
            lower_bound + (self.x - lower_bound.wrapping_sub(rhs.x)) % (upper_bound - lower_bound);
        let y =
            lower_bound + (self.y - lower_bound.wrapping_sub(rhs.y)) % (upper_bound - lower_bound);
        Self { x, y }
    }

    pub fn checked_add(self, rhs: Self, range: impl RangeBounds<isize>) -> Option<Self> {
        let x = self.x.checked_add(rhs.x)?;
        let y = self.y.checked_add(rhs.y)?;
        if !range.contains(&x) || !range.contains(&y) {
            return None;
        }
        Some(Self { x, y })
    }

    pub fn checked_sub(self, rhs: Self, range: impl RangeBounds<isize>) -> Option<Self> {
        let x = self.x.checked_sub(rhs.x)?;
        let y = self.y.checked_sub(rhs.y)?;
        if !range.contains(&x) || !range.contains(&y) {
            return None;
        }
        Some(Self { x, y })
    }

    pub fn is_in_bounds(self, range: impl RangeBounds<isize>) -> bool {
        range.contains(&self.x) && range.contains(&self.y)
    }

    pub fn manhattan_distance(self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn distance(self, other: Self) -> f32 {
        (self.x.abs_diff(other.x) as f32 + self.y.abs_diff(other.y) as f32).sqrt()
    }

    /// Assumes X is right and Y is down.
    pub fn turn_clockwise(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Assumes X is right and Y is down.
    pub fn turn_counterclockwise(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<(isize, isize)> for Coordinate {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self::Display::fmt(&self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapping_add() {
        assert_eq!(
            Coordinate::new(3, 3).wrapping_add(Coordinate::new(5, 5), -5..=5),
            Coordinate::new(-2, -2)
        );
    }

    #[test]
    fn test_wrapping_sub() {
        assert_eq!(
            Coordinate::new(-3, -3).wrapping_sub(Coordinate::new(5, 5), -5..=5),
            Coordinate::new(2, 2)
        );
    }
}
