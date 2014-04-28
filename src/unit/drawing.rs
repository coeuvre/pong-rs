use super::{Unit, ToUnit};

pub struct Pixel(pub int);

impl Pixel {
    #[inline]
    pub fn val(&self) -> int {
        let Pixel(v) = *self;
        v
    }
}

pub trait ToPixel {
    fn to_pixel(&self) -> Pixel;
}

impl ToPixel for int {
    fn to_pixel(&self) -> Pixel {
        Pixel(*self)
    }
}

impl ToPixel for Pixel {
    fn to_pixel(&self) -> Pixel {
        *self
    }
}

impl ToUnit for Pixel {
    fn to_unit(&self) -> Unit {
        let Pixel(a) = *self;
        Unit(a as f32)
    }
}

impl<T: ToPixel> Sub<T, Pixel> for Pixel {
    fn sub(&self, rhs: &T) -> Pixel {
        let (Pixel(a), Pixel(b)) = (*self, rhs.to_pixel());
        Pixel(a - b)
    }
}

impl<T: ToPixel> Div<T, Pixel> for Pixel {
    fn div(&self, rhs: &T) -> Pixel {
        let (Pixel(a), Pixel(b)) = (*self, rhs.to_pixel());
        Pixel(a / b)
    }
}


pub struct Point {
    pub x: Pixel,
    pub y: Pixel,
}

impl Point {
    pub fn new<A: ToPixel, B: ToPixel>(x: A, y: B) -> Point {
        Point {
            x: x.to_pixel(),
            y: y.to_pixel(),
        }
    }
}

impl Sub<Point, Point> for Point {
    fn sub(&self, rhs: &Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}


pub struct Size {
    pub w: Pixel,
    pub h: Pixel,
}

impl Size {
    #[inline]
    pub fn new<A: ToPixel, B: ToPixel>(w: A, h: B) -> Size {
        Size {
            w: w.to_pixel(),
            h: h.to_pixel(),
        }
    }
}
