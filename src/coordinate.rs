#![allow(dead_code)]

use ahash::HashMap;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, Bound, RangeBounds, Sub, SubAssign},
};

/// A two-dimensional coordinate, supporting negative coordinates.
#[derive(Copy, Clone, Default, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Parses a string using rows and columns as coordinates, with the origin at the top left, and
    /// returns a map of all coordinates for which the processing function returns `Some(T)`,
    /// allowing for sparse coordinate systems.
    pub fn parse_grid<T>(s: &str, f: fn(char) -> Option<T>) -> HashMap<Self, T> {
        s.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    f(c).map(|t| (Coordinate::new(x as isize, y as isize), t))
                })
            })
            .collect()
    }

    /// Addition that wraps around the edges of the grid.
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

    /// Subtraction that wraps around the edges of the grid.
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

    /// Addition that fails if the result is outside `range`.
    pub fn checked_add(self, rhs: Self, range: impl RangeBounds<isize>) -> Option<Self> {
        let x = self.x.checked_add(rhs.x)?;
        let y = self.y.checked_add(rhs.y)?;
        if !range.contains(&x) || !range.contains(&y) {
            return None;
        }
        Some(Self { x, y })
    }

    /// Subtraction that fails if the result is outside `range`.
    pub fn checked_sub(self, rhs: Self, range: impl RangeBounds<isize>) -> Option<Self> {
        let x = self.x.checked_sub(rhs.x)?;
        let y = self.y.checked_sub(rhs.y)?;
        if !range.contains(&x) || !range.contains(&y) {
            return None;
        }
        Some(Self { x, y })
    }

    /// Returns `true` if `self` is within `range` on both axes. Inherently only works on square
    /// grids.
    pub fn is_in_bounds(self, range: impl RangeBounds<isize>) -> bool {
        range.contains(&self.x) && range.contains(&self.y)
    }

    /// Returns the manhattan distance to `other`.
    pub const fn manhattan_distance(self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    /// Returns the distance to `other`.
    pub fn distance(self, other: Self) -> f32 {
        (self.x.abs_diff(other.x) as f32 + self.y.abs_diff(other.y) as f32).sqrt()
    }

    /// Treating `self` as a relative coordinate, turns clockwise. Assumes X is right and Y is down.
    pub const fn turn_clockwise(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Treating `self` as a relative coordinate, turns counter-clockwise. Assumes X is right and Y
    /// is down.
    pub const fn turn_counterclockwise(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    /// Returns all neighbours (up, down, left, right) that are within bounds.
    pub fn neighbours(self, range: impl RangeBounds<isize> + Clone) -> Vec<Self> {
        DIRECTIONS
            .iter()
            .map(|n| (n, range.clone()))
            .filter_map(|(n, range)| self.checked_add(*n, range))
            .collect()
    }

    /// Returns all neighbours, including diagonal ones, that are within bounds.
    pub fn diagonal_neighbours(self, range: impl RangeBounds<isize> + Clone) -> Vec<Self> {
        DIAGONAL_DIRECTIONS
            .iter()
            .map(|n| (n, range.clone()))
            .filter_map(|(n, range)| self.checked_add(*n, range))
            .collect()
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

/// A relative up coordinate, assuming origin in the top left.
pub const UP: Coordinate = Coordinate::new(0, -1);
/// A relative down coordinate, assuming origin in the top left.
pub const DOWN: Coordinate = Coordinate::new(0, 1);
/// A relative left coordinate, assuming origin in the top left.
pub const LEFT: Coordinate = Coordinate::new(-1, 0);
/// A relative right coordinate, assuming origin in the top left.
pub const RIGHT: Coordinate = Coordinate::new(1, 0);

/// The four cardinal directions.
pub const DIRECTIONS: [Coordinate; 4] = [UP, DOWN, LEFT, RIGHT];

/// The eight directions including diagonals.
pub const DIAGONAL_DIRECTIONS: [Coordinate; 8] = [
    Coordinate::new(-1, -1),
    Coordinate::new(-1, 0),
    Coordinate::new(-1, 1),
    Coordinate::new(0, -1),
    Coordinate::new(0, 1),
    Coordinate::new(1, -1),
    Coordinate::new(1, 0),
    Coordinate::new(1, 1),
];

#[cfg(test)]
mod tests {
    use super::*;
    use ahash::HashSet;

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

    #[test]
    fn test_neighbours() {
        assert_eq!(
            HashSet::from_iter(Coordinate::new(1, 1).neighbours(0..=2)),
            HashSet::from_iter([(0, 1), (1, 0), (1, 2), (2, 1)].map(Coordinate::from))
        );
    }

    #[test]
    fn test_neighbours_with_range() {
        assert_eq!(
            HashSet::from_iter(Coordinate::new(1, 1).neighbours(1..=2)),
            HashSet::from_iter([(1, 2), (2, 1)].map(Coordinate::from))
        );
    }
}
