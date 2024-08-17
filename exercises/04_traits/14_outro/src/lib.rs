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

macro_rules! saturating_u16 {
    ($type:ty) => {
        impl From<$type> for SaturatingU16 {
            fn from(value: $type) -> SaturatingU16 {
                Self { value: value as u16 }
            }
        }

        impl From<&$type> for SaturatingU16 {
            fn from(value: &$type) -> SaturatingU16 {
                value.clone().into()
            }
        }

        impl Add<$type> for SaturatingU16 {
            type Output = SaturatingU16;

            fn add(self, rhs: $type) -> SaturatingU16 {
                let (sum, overflow) = self.value.overflowing_add(rhs as u16);
                if overflow {
                    Self::from(u16::MAX)
                } else {
                    Self::from(sum)
                }
            }
        }

        impl Add<&$type> for SaturatingU16 {
            type Output = SaturatingU16;

            fn add(self, rhs: &$type) -> SaturatingU16 {
                self + *rhs
            }
        }

        impl PartialEq<$type> for SaturatingU16 {
            fn eq(&self, other: &$type) -> bool {
                self.value == *other as u16
            }
        }

        impl PartialEq<SaturatingU16> for $type {
            fn eq(&self, other: &SaturatingU16) -> bool {
                *self as u16 == other.value
            }
        }
    };
}

saturating_u16!(u16);
saturating_u16!(u8);
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SaturatingU16 {
    value: u16,
}

impl Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> SaturatingU16 {
        let (sum, overflow) = self.value.overflowing_add(rhs.value);
        if overflow {
            Self::from(u16::MAX)
        } else {
            Self::from(sum)
        }
    }
}
