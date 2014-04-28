pub use self::drawing::{Pixel, ToPixel, Point, Size};
pub use self::physics::{Vec2, ToVec2, AABB, MS};

pub mod drawing;
pub mod physics;

pub mod vec2 {
    use super::{Unit, Vec2};

    pub static ZERO: Vec2 = Vec2 {
        x: Unit(0.0),
        y: Unit(0.0),
    };
}

pub static ZERO: Unit = Unit(0.0);

#[deriving(Eq, Ord)]
pub struct Unit(pub f32);

impl Unit {
    pub fn val(&self) -> f32 {
        let Unit(a) = *self;
        a
    }
}

pub trait ToUnit {
    fn to_unit(&self) -> Unit;
}

impl ToUnit for f32 {
    fn to_unit(&self) -> Unit {
        Unit(*self)
    }
}

impl ToUnit for f64 {
    fn to_unit(&self) -> Unit {
        Unit(*self as f32)
    }
}

impl ToUnit for Unit {
    fn to_unit(&self) -> Unit {
        *self
    }
}

impl ToPixel for Unit {
    fn to_pixel(&self) -> Pixel {
        Pixel(self.val().round() as int)
    }
}

impl Neg<Unit> for Unit {
    fn neg(&self) -> Unit {
        let Unit(a) = *self;
        Unit(-a)
    }
}

impl<T: ToUnit> Add<T, Unit> for Unit {
    fn add(&self, rhs: &T) -> Unit {
        let (Unit(a), Unit(b)) = (*self, rhs.to_unit());
        Unit(a + b)
    }
}

impl<T: ToUnit> Sub<T, Unit> for Unit {
    fn sub(&self, rhs: &T) -> Unit {
        let (Unit(a), Unit(b)) = (*self, rhs.to_unit());
        Unit(a - b)
    }
}

impl<T: ToUnit> Mul<T, Unit> for Unit {
    fn mul(&self, rhs: &T) -> Unit {
        let (Unit(a), Unit(b)) = (*self, rhs.to_unit());
        Unit(a * b)
    }
}

impl<T: ToUnit> Div<T, Unit> for Unit {
    fn div(&self, rhs: &T) -> Unit {
        let (Unit(a), Unit(b)) = (*self, rhs.to_unit());
        Unit(a / b)
    }
}
