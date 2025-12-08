#![allow(dead_code)]

use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, Bound, RangeBounds, Sub, SubAssign},
};

/// A two-dimensional coordinate, supporting negative coordinates.
#[derive(Copy, Clone, Default, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Coordinate {
    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
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
        let z =
            lower_bound + (self.z - lower_bound.wrapping_add(rhs.z)) % (upper_bound - lower_bound);
        Self { x, y, z }
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
        let z =
            lower_bound + (self.z - lower_bound.wrapping_sub(rhs.z)) % (upper_bound - lower_bound);
        Self { x, y, z }
    }

    /// Addition that fails if the result is outside `range`.
    pub fn checked_add(self, rhs: Self, range: impl RangeBounds<isize>) -> Option<Self> {
        let x = self.x.checked_add(rhs.x)?;
        let y = self.y.checked_add(rhs.y)?;
        let z = self.z.checked_add(rhs.z)?;
        if !range.contains(&x) || !range.contains(&y) || !range.contains(&z) {
            return None;
        }
        Some(Self { x, y, z })
    }

    /// Subtraction that fails if the result is outside `range`.
    pub fn checked_sub(self, rhs: Self, range: impl RangeBounds<isize>) -> Option<Self> {
        let x = self.x.checked_sub(rhs.x)?;
        let y = self.y.checked_sub(rhs.y)?;
        let z = self.z.checked_sub(rhs.z)?;
        if !range.contains(&x) || !range.contains(&y) || !range.contains(&z) {
            return None;
        }
        Some(Self { x, y, z })
    }

    /// Returns `true` if `self` is within `range` on both axes. Inherently only works on square
    /// grids.
    pub fn is_in_bounds(self, range: impl RangeBounds<isize>) -> bool {
        range.contains(&self.x) && range.contains(&self.y)
    }

    /// Returns the manhattan distance to `other`.
    pub const fn manhattan_distance(self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }

    /// Returns the distance to `other`.
    pub fn distance(self, other: Self) -> f32 {
        ((self.x.abs_diff(other.x) as f32).powi(2)
            + (self.y.abs_diff(other.y) as f32).powi(2)
            + (self.z.abs_diff(other.z) as f32).powi(2))
        .sqrt()
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
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl From<(isize, isize, isize)> for Coordinate {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Self { x, y, z }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{},{})", self.x, self.y, self.z))
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self::Display::fmt(&self, f)
    }
}

/// A relative up coordinate, assuming origin in the top left.
pub const UP: Coordinate = Coordinate::new(0, -1, 0);
/// A relative down coordinate, assuming origin in the top left.
pub const DOWN: Coordinate = Coordinate::new(0, 1, 0);
/// A relative left coordinate, assuming origin in the top left.
pub const LEFT: Coordinate = Coordinate::new(-1, 0, 0);
/// A relative right coordinate, assuming origin in the top left.
pub const RIGHT: Coordinate = Coordinate::new(1, 0, 0);
pub const IN: Coordinate = Coordinate::new(0, 0, 1);
pub const OUT: Coordinate = Coordinate::new(0, 0, -1);

/// The four cardinal directions.
pub const DIRECTIONS: [Coordinate; 6] = [UP, DOWN, LEFT, RIGHT, IN, OUT];

/// The eight directions including diagonals.
pub const DIAGONAL_DIRECTIONS: [Coordinate; 26] = [
    Coordinate::new(-1, -1, -1),
    Coordinate::new(0, -1, -1),
    Coordinate::new(1, -1, -1),
    Coordinate::new(-1, 0, -1),
    Coordinate::new(0, 0, -1),
    Coordinate::new(1, 0, -1),
    Coordinate::new(-1, 1, -1),
    Coordinate::new(0, 1, -1),
    Coordinate::new(1, 1, -1),
    Coordinate::new(-1, -1, 0),
    Coordinate::new(0, -1, 0),
    Coordinate::new(1, -1, 0),
    Coordinate::new(-1, 0, 0),
    Coordinate::new(1, 0, 0),
    Coordinate::new(-1, 1, 0),
    Coordinate::new(0, 1, 0),
    Coordinate::new(1, 1, 0),
    Coordinate::new(-1, -1, 1),
    Coordinate::new(0, -1, 1),
    Coordinate::new(1, -1, 1),
    Coordinate::new(-1, 0, 1),
    Coordinate::new(0, 0, 1),
    Coordinate::new(1, 0, 1),
    Coordinate::new(-1, 1, 1),
    Coordinate::new(0, 1, 1),
    Coordinate::new(1, 1, 1),
];
