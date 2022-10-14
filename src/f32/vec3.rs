use core::ops::{
  Add,
  Sub,
  Neg,
};
use core::fmt::{Display};
use std::fmt::{Formatter};

// 3-dimensional vector.
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

// Todo: Implement PartialEq<_>

impl Vec3 {
  /// Creates a 3-dimensional vector.
  pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
    Self { x, y, z }
  }

  /// Takes the dot product of two vectors.
  #[inline]
  pub fn dot(self, rhs: Self) -> f32 {
    (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
  }

  /// Takes the cross (vector) product of two vectors.
  #[inline]
  pub fn cross(self, rhs: Self) -> Self {
    Self {
      x: self.y * rhs.z - self.z * rhs.y,
      y: self.z * rhs.x - self.x * rhs.z,
      z: self.x * rhs.y - self.y * rhs.x,
    }
  }
}

impl Add<Vec3> for Vec3 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl Sub<Vec3> for Vec3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl Display for Vec3 {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{{}, {}, {}}}", self.x, self.y, self.z)
  }
}