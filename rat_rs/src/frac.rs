//! frac是本lib最关键的数据结构，核心是Fraction结构体，字段包括分子numer，分母denom和符号sign
//!
//! # Example
//! ```rust
//! use rat_rs::frac::{Fraction, FractionSign};
//! let f = Fraction::new(1, 2, FractionSign::NonNegative).unwrap();
//! let g = Fraction::with_negative(1, 2).unwrap();
//! assert_eq!(f + g, 0);
//! ```
use core::ops::Neg;

/// 分数
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Fraction {
    numer: u32,
    denom: u32,
    sign: FractionSign,
}

/// 符号
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FractionSign {
    NonNegative = 0,
    Negative = 1,
}

impl Neg for FractionSign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self == Self::Negative {
            Self::NonNegative
        } else {
            Self::Negative
        }
    }
}

impl From<u8> for FractionSign {
    fn from(value: u8) -> Self {
        match value {
            0 => FractionSign::NonNegative,
            1 => FractionSign::Negative,
            _ => panic!("invalid sign"),
        }
    }
}

impl PartialEq<u32> for Fraction {
    fn eq(&self, other: &u32) -> bool {
        self.numer() == *other && self.denom() == 1
    }
}

impl Fraction {
    pub fn new(numer: u32, denom: u32, sign: FractionSign) -> Option<Self> {
        if denom == 0 {
            return None;
        }
        let gcd = gcd(numer.into(), denom.into());
        Some(Self {
            numer: u32::try_from(numer as u64 / gcd).ok()?,
            denom: u32::try_from(denom as u64 / gcd).ok()?,
            sign,
        })
    }

    pub fn with_non_negative(numer: u32, denom: u32) -> Option<Self> {
        Self::new(numer, denom, FractionSign::NonNegative)
    }

    pub fn with_negative(numer: u32, denom: u32) -> Option<Self> {
        Self::new(numer, denom, FractionSign::Negative)
    }

    pub fn numer(&self) -> u32 {
        self.numer
    }

    pub fn denom(&self) -> u32 {
        self.denom
    }

    pub fn sign(&self) -> FractionSign {
        self.sign
    }
}

/// 计算两个数的最大公约数
pub(crate) fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = core::mem::replace(&mut b, remainder);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_with_corner_cases() {
        assert_eq!(gcd(42, 12), 6);
        assert_eq!(gcd(12, 42), 6);
        assert_eq!(gcd(100, 0), 100);
        assert_eq!(gcd(0, 100), 100);
        assert_eq!(gcd(37, 73), 1);
        assert_eq!(gcd(42, 1), 1);
        assert_eq!(gcd(1, 42), 1);
    }

    #[test]
    fn test_new_fraction_with_corner_cases() {
        assert_eq!(
            Fraction::new(42, 12, FractionSign::NonNegative),
            Some(Fraction {
                numer: 7,
                denom: 2,
                sign: FractionSign::NonNegative
            })
        );
        assert_eq!(
            Fraction::new(12, 42, FractionSign::NonNegative),
            Some(Fraction {
                numer: 2,
                denom: 7,
                sign: FractionSign::NonNegative
            })
        );
        assert_eq!(Fraction::new(100, 0, FractionSign::NonNegative), None);
        assert_eq!(
            Fraction::new(0, 100, FractionSign::NonNegative),
            Some(Fraction {
                numer: 0,
                denom: 1,
                sign: FractionSign::NonNegative
            })
        );
        assert_eq!(
            Fraction::new(37, 73, FractionSign::NonNegative),
            Some(Fraction {
                numer: 37,
                denom: 73,
                sign: FractionSign::NonNegative
            })
        );
        assert_eq!(
            Fraction::new(42, 1, FractionSign::NonNegative),
            Some(Fraction {
                numer: 42,
                denom: 1,
                sign: FractionSign::NonNegative
            })
        );
        assert_eq!(
            Fraction::new(42, 12, FractionSign::NonNegative),
            Some(Fraction {
                numer: 7,
                denom: 2,
                sign: FractionSign::NonNegative
            })
        );
        assert_eq!(
            Fraction::new(1, 42, FractionSign::NonNegative),
            Some(Fraction {
                numer: 1,
                denom: 42,
                sign: FractionSign::NonNegative
            })
        );
    }
}
