use std::f32::consts::{FRAC_PI_4, PI};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use rand::rngs::ThreadRng;
use rand::Rng;
use sdl2::rect::Point;

use crate::game_and_scene::scene::{SCREEN_HEIGHT, SCREEN_WIDTH};

/// This constant determines the default launching angle of the ball. For example, if the angle is
/// $\frac{\pi}{4}$, you can imagine that the screen is divided into 4 regions by lines $y=x$ and
/// $y=-x$. The balls will only be launched to the left and right regions.
pub const DEFAULT_ANGLE_RANGE: f32 = FRAC_PI_4;

/// The vec2 struct. Not implemented as a generic struct since some operations are not supported by
/// integer types.
#[derive(Debug, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Constructor.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Generate a random vec2 given its magnitude.
    pub fn random_with_magnitude(magnitude: f32, range: Option<f32>, rng: &mut ThreadRng) -> Self {
        let bound: f32 = match range {
            Some(range) => range,
            None => DEFAULT_ANGLE_RANGE,
        };

        let direction: f32 = rng.gen_range(-bound..bound);
        let side: f32 = if rng.gen_bool(1.0 / 2.0) { -1.0 } else { 1.0 };

        Self {
            x: magnitude * direction.cos() * side,
            y: magnitude * direction.sin(),
        }
    }
}

impl Into<Point> for &Vec2 {
    /// Supports conversion to sdl2 points.
    fn into(self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }
}

impl Add for &Vec2 {
    type Output = Vec2;

    /// Supports operator `+`.
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for &Vec2 {
    type Output = Vec2;

    /// Supports operator `-`.
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign<&Self> for Vec2 {
    /// Supports operator `+=`.
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<&Self> for Vec2 {
    /// Supports operator `-=`.
    fn sub_assign(&mut self, rhs: &Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for &Vec2 {
    type Output = Vec2;

    /// Supports operator `*`. Notice that this multiplication is different from dot or cross
    /// products between vectors. This is just component-by-component multiplication.
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for &Vec2 {
    type Output = Vec2;

    /// Supports operator `/`. Notice that division between 2D vectors are not properly defined in
    /// math. This is just component-by-component division.
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    /// Supports operator `*=`.
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign<f32> for Vec2 {
    /// Supports operator `/=`.
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Mul<&Vec2> for f32 {
    type Output = Vec2;

    /// Supports scalar multiplication.
    fn mul(self, rhs: &Vec2) -> Self::Output {
        rhs * self
    }
}

impl Default for Vec2 {
    /// Generates a default vec2. A default vec2 is in the center of the screen.
    fn default() -> Self {
        Self {
            x: (SCREEN_WIDTH / 2) as f32,
            y: (SCREEN_HEIGHT / 2) as f32,
        }
    }
}
