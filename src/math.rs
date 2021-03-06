
// calculations inspired by
// https://github.com/AcademySoftwareFoundation/openexr/blob/master/OpenEXR/IlmImf/ImfTiledMisc.cpp


use std::convert::TryFrom;
use crate::error::{i32_to_u32, i32_to_usize};
use crate::error::Result;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2<T> (pub T, pub T);

impl<T> Vec2<T> {
    pub fn map<B>(self, map: impl Fn(T) -> B) -> Vec2<B> {
        Vec2(map(self.0), map(self.1))
    }

    pub fn try_from<S>(value: Vec2<S>) -> std::result::Result<Self, T::Error> where T: TryFrom<S> {
        let x = T::try_from(value.0)?;
        let y = T::try_from(value.1)?;
        Ok(Vec2(x, y))
    }

    pub fn area(self) -> T where T: std::ops::Mul<T, Output = T> {
        self.0 * self.1
    }
}



impl Vec2<i32> {
    pub fn to_usize(self) -> Result<Vec2<usize>> {
        let x = i32_to_usize(self.0)?;
        let y = i32_to_usize(self.1)?;
        Ok(Vec2(x, y))
    }

    pub fn to_u32(self) -> Result<Vec2<u32>> {
        let x = i32_to_u32(self.0)?;
        let y = i32_to_u32(self.1)?;
        Ok(Vec2(x, y))
    }
}

impl<T: std::ops::Add<T>> std::ops::Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn add(self, other: Vec2<T>) -> Self::Output {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: std::ops::Sub<T>> std::ops::Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn sub(self, other: Vec2<T>) -> Self::Output {
        Vec2(self.0 - other.0, self.1 - other.1)
    }
}

impl<T: std::ops::Div<T>> std::ops::Div<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn div(self, other: Vec2<T>) -> Self::Output {
        Vec2(self.0 / other.0, self.1 / other.1)
    }
}

impl<T: std::ops::Mul<T>> std::ops::Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn mul(self, other: Vec2<T>) -> Self::Output {
        Vec2(self.0 * other.0, self.1 * other.1)
    }
}


/// computes floor(log(x)/log(2))
pub fn floor_log_2(mut number: u32) -> u32 {
    debug_assert_ne!(number, 0);

    let mut log = 0;

//     TODO check if this unrolls properly?
    while number > 1 {
        log += 1;
        number >>= 1;
    }

    log
}


/// computes ceil(log(x)/log(2))
// taken from https://github.com/openexr/openexr/blob/master/OpenEXR/IlmImf/ImfTiledMisc.cpp
pub fn ceil_log_2(mut number: u32) -> u32 {
    debug_assert_ne!(number, 0);

    let mut log = 0;
    let mut round_up = 0;

    // TODO check if this unrolls properly
    while number > 1 {
        if number & 1 != 0 {
            round_up = 1;
        }

        log +=  1;
        number >>= 1;
    }

    log + round_up
}



#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RoundingMode {
    Down, Up,
}

impl RoundingMode {
    pub fn log2(self, number: u32) -> u32 {
        match self {
            RoundingMode::Down => self::floor_log_2(number),
            RoundingMode::Up => self::ceil_log_2(number),
        }
    }

    pub fn divide(self, dividend: u32, divisor: u32) -> u32 {
        match self {
            RoundingMode::Up => (dividend + divisor - 1) / divisor, // only works for positive numbers
            RoundingMode::Down => dividend / divisor,
        }
    }
}
