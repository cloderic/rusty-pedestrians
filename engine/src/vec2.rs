use serde::Serialize;
use std::fmt;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Vec2 {
  x: f64,
  y: f64,
}

impl Vec2 {
  pub fn new(x: f64, y: f64) -> Self {
    Vec2 { x, y }
  }

  pub fn x(&self) -> f64 {
    self.x
  }

  pub fn y(&self) -> f64 {
    self.y
  }

  pub fn sqr_norm(&self) -> f64 {
    self.x.powi(2) + self.y.powi(2)
  }

  pub fn norm(&self) -> f64 {
    self.sqr_norm().sqrt()
  }

  pub fn normalize_to(&self, new_norm: f64) -> Self {
    let norm = self.norm();
    if norm == 0. && new_norm == 0. {
      Vec2::new(0., 0.)
    } else {
      let factor = new_norm / self.norm();
      Vec2::new(self.x * factor, self.y * factor)
    }
  }

  pub fn normalize(&self) -> Self {
    self.normalize_to(1.)
  }

  pub fn cap_norm(&self, cap: f64) -> Self {
    let sqr_norm = self.sqr_norm();
    if cap.powi(2) >= sqr_norm {
      *self
    } else if sqr_norm > 0. && cap > 0. {
      let factor = cap / sqr_norm.sqrt();
      Vec2::new(self.x * factor, self.y * factor)
    } else {
      Vec2::new(0., 0.)
    }
  }

  pub fn det(v1: Self, v2: Self) -> f64 {
    v1.x * v2.y - v1.y * v2.x
  }
}

impl Default for Vec2 {
  fn default() -> Self {
    Vec2::new(0., 0.)
  }
}

impl fmt::Display for Vec2 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl Add for Vec2 {
  type Output = Vec2;

  fn add(self, other: Vec2) -> Vec2 {
    Vec2 {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl Sub for Vec2 {
  type Output = Self;

  fn sub(self, other: Vec2) -> Vec2 {
    Vec2 {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl Neg for Vec2 {
  type Output = Self;

  fn neg(self) -> Vec2 {
    Vec2 {
      x: -self.x,
      y: -self.y,
    }
  }
}

impl Mul<f64> for Vec2 {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self {
    Vec2 {
      x: self.x * scalar,
      y: self.y * scalar,
    }
  }
}

impl MulAssign<f64> for Vec2 {
  fn mul_assign(&mut self, scalar: f64) {
    self.x *= scalar;
    self.y *= scalar;
  }
}

impl Mul<Vec2> for f64 {
  type Output = Vec2;

  fn mul(self, vector: Vec2) -> Vec2 {
    Vec2 {
      x: vector.x * self,
      y: vector.y * self,
    }
  }
}

impl Mul<Vec2> for Vec2 {
  type Output = f64;

  fn mul(self, other: Vec2) -> f64 {
    self.x * other.x + self.y * other.y
  }
}

impl Div<f64> for Vec2 {
  type Output = Self;

  fn div(self, scalar: f64) -> Self {
    Vec2 {
      x: self.x / scalar,
      y: self.y / scalar,
    }
  }
}

impl DivAssign<f64> for Vec2 {
  fn div_assign(&mut self, scalar: f64) {
    self.x /= scalar;
    self.y /= scalar;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use assert_approx_eq::assert_approx_eq;

  #[test]
  fn test_new() {
    let v = Vec2::new(1.0, 2.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
  }

  #[test]
  fn test_add() {
    assert_eq!(
      Vec2::new(1.0, 2.0) + Vec2::new(4.0, 5.0),
      Vec2::new(5.0, 7.0)
    );
  }

  #[test]
  fn test_sub() {
    assert_eq!(
      Vec2::new(1.0, 2.0) - Vec2::new(4.0, 5.0),
      Vec2::new(-3.0, -3.0)
    );
  }

  #[test]
  fn test_mul_scalar() {
    assert_eq!(Vec2::new(1.0, 2.0) * 2.0, Vec2::new(2.0, 4.0));
    assert_eq!(-1. * Vec2::new(1.0, 2.0), Vec2::new(-1.0, -2.0));
  }

  #[test]
  fn test_cap_norm() {
    assert_approx_eq!(Vec2::new(1.0, 2.0).cap_norm(0.5).norm(), 0.5);
    assert_approx_eq!(Vec2::new(0., 0.).cap_norm(0.5).norm(), 0.);
  }
}
