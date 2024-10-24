// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SaturatingU16 {
    value: u16
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        *other == self.value
    }
}

macro_rules! generate_from {
    (
        $($integer:ty), +
    ) => {
        $(
            impl From<$integer> for SaturatingU16 {
                fn from(value: $integer) -> Self {
                    let value = value as u16;
                    SaturatingU16{value}
                }
            }
    
            impl From<&$integer> for SaturatingU16 {
                fn from(value: &$integer) -> Self {
                    let value = (*value) as u16;
                    SaturatingU16{value}
                }
            }
        )+
    };
}

impl Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: Self) -> Self::Output {
        Self{value: self.value.saturating_add(rhs.value)}
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &Self) -> Self::Output {
        Self{value: self.value.saturating_add(rhs.value)}
    }
}

macro_rules! generate_add {
    (
        $($integer:ty), +
    ) => {
        $(
            impl Add<$integer> for SaturatingU16 {
                type Output = SaturatingU16;

                fn add(self, rhs: $integer) -> Self::Output {
                    Self{value: self.value.saturating_add(rhs.into())}
                }
            }

            impl Add<&$integer> for SaturatingU16 {
                type Output = SaturatingU16;

                fn add(self, rhs: &$integer) -> Self::Output {
                    Self{value: self.value.saturating_add((*rhs).into())}
                }
            }
        )+
    };
}

generate_from!(u16, u8);
generate_add!(u16, u8);
