use std::ops::{ Mul, Add, Sub, Div };
use std::cmp::max;
use num_traits::cast::ToPrimitive;
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Uvector {
    pub x: usize,
    pub y: usize,
}
impl<T: ToPrimitive> Mul<T> for Uvector{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.to_usize().unwrap();
        Self{
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Mul for Uvector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl<T: ToPrimitive> Div<T> for Uvector {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.to_usize().unwrap();
        Self{
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl Add for Uvector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Uvector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ivector {
    pub x: isize,
    pub y: isize,
}
impl Ivector {
    pub fn abs(&self) -> Ivector{
        Ivector {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}
impl<T: ToPrimitive> Mul<T> for Ivector{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.to_isize().unwrap();
        Self{
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Mul for Ivector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl<T: ToPrimitive> Div<T> for Ivector {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.to_isize().unwrap();
        Self{
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl Add for Ivector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Ivector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Add<Uvector> for Ivector {
   type Output = Self; 

    fn add(self, rhs: Uvector) -> Self::Output {
        Self {
            x: self.x + rhs.x as isize,
            y: self.y + rhs.y as isize,
        }
    }
}
impl Sub<Uvector> for Ivector {
   type Output = Self; 

    fn sub(self, rhs: Uvector) -> Self::Output {
        Self {
            x: self.x - rhs.x as isize,
            y: self.y - rhs.y as isize,
        }
    }
}
impl Add<Ivector> for Uvector {
    type Output = Ivector;

    fn add(self, rhs: Ivector) -> Self::Output {
        Ivector {
            x: self.x as isize + rhs.x,
            y: self.y as isize + rhs.y,
        }
    }
}
impl Sub<Ivector> for Uvector {
    type Output = Ivector;

    fn sub(self, rhs: Ivector) -> Self::Output {
        Ivector {
            x: self.x as isize - rhs.x,
            y: self.y as isize - rhs.y,
        }
    }
}
impl From<Uvector> for Ivector {
    fn from(uvector: Uvector) -> Self {
        Self {
            x: uvector.x as isize,
            y: uvector.y as isize,
        }
    }
}
impl From<Ivector> for Uvector {
    fn from(ivector: Ivector) -> Self {
        let Ivector { x, y } = ivector;
        Self {
            x: max(x, 0) as usize,
            y: max(y, 0) as usize,
        }
    }
}