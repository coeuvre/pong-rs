use super::{Unit, ToUnit};

#[deriving(Eq, Ord)]
pub struct Vec2 {
    pub x: Unit,
    pub y: Unit,
}

impl Vec2 {
    pub fn new<A: ToUnit, B: ToUnit>(x: A, y: B) -> Vec2 {
        Vec2 {
            x: x.to_unit(),
            y: y.to_unit(),
        }
    }

    pub fn norm(&self) -> Vec2 {
        let len = self.length();
        Vec2::new(self.x / len, self.y / len)
    }

    pub fn length(&self) -> Unit {
        let x = self.x.val();
        let y = self.y.val();
        Unit((x * x + y * y).sqrt())
    }
}

pub trait ToVec2 {
    fn to_vec(&self) -> Vec2;
}

impl ToVec2 for Vec2 {
    fn to_vec(&self) -> Vec2 {
        *self
    }
}

impl Add<Vec2, Vec2> for Vec2 {
    fn add(&self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: ToUnit> Mul<T, Vec2> for Vec2 {
    fn mul(&self, rhs: &T) -> Vec2 {
        let a = rhs.to_unit();
        Vec2 {
            x: self.x * a,
            y: self.y * a,
        }
    }
}


/// x axis from left to right and y asix from top to bottom
pub struct AABB {
    pub center: Vec2,
    pub size: Vec2,
}

impl AABB {
    pub fn new<A: ToUnit, B: ToUnit, C: ToUnit, D: ToUnit>(x: A, y: B, w: C, h: D) -> AABB {
        AABB {
            center: Vec2::new(x, y),
            size: Vec2::new(w, h),
        }
    }

    pub fn transform(&self, offset: Vec2) -> AABB {
        AABB {
            center: self.center + offset,
            size: self.size,
        }
    }

    pub fn is_collided_with(&self, other: &AABB) -> bool {
        self.right() >= other.left() &&
        self.left() <= other.right() &&
        self.top() <= other.bottom() &&
        self.bottom() >= other.top()
    }

    pub fn left(&self) -> Unit {
        self.center.x - self.size.x / 2.0
    }

    pub fn right(&self) -> Unit {
        self.center.x + self.size.x / 2.0
    }

    pub fn top(&self) -> Unit {
        self.center.y - self.size.y / 2.0
    }

    pub fn bottom(&self) -> Unit {
        self.center.y + self.size.y / 2.0
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }
}

#[deriving(Eq, Ord)]
pub struct MS(pub uint);

impl MS {
    pub fn val(&self) -> uint {
        let MS(a) = *self;
        a
    }
}

impl ToUnit for MS {
    fn to_unit(&self) -> Unit {
        let MS(a) = *self;
        Unit(a as f32)
    }
}

impl Sub<MS, MS> for MS {
    fn sub(&self, rhs: &MS) -> MS {
        MS(self.val() - rhs.val())
    }
}

impl Add<MS, MS> for MS {
    fn add(&self, rhs: &MS) -> MS {
        MS(self.val() + rhs.val())
    }
}
