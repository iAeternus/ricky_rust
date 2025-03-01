use crate::frac::{Fraction, FractionSign};

macro_rules! primitive_unsign_conv {
    ($($unsign: ty),*) => {
        $(
            impl From<$unsign> for Fraction {
                fn from(value: $unsign) -> Self {
                    Self::new(value.into(), 1, FractionSign::NonNegative).unwrap()
                }
            }
        ) *
    };
}

primitive_unsign_conv!(u8, u16, u32);

macro_rules! primitive_sign_conv {
    ($($sign: ty),*) => {
       $(
           impl From<$sign> for Fraction {
               fn from(value: $sign) -> Self {
                   if value < 0 {
                       Self::new(-(value as i64) as u32, 1, FractionSign::Negative).unwrap()
                   } else {
                       Self::new(value as u32, 1, FractionSign::NonNegative).unwrap()
                   }
               }
           }
       )*
    };
}

primitive_sign_conv!(i8, i16, i32);

impl TryFrom<u64> for Fraction {
    type Error = u64;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > u32::MAX.into() {
            return Err(value);
        }
        Ok(Self::from(value as u32))
    }
}

impl TryFrom<u128> for Fraction {
    type Error = u128;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value > u32::MAX.into() {
            return Err(value);
        }
        Ok(Self::from(value as u32))
    }
}

impl TryFrom<i64> for Fraction {
    type Error = i64;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value > u32::MAX.into() || value < -(i64::from(u32::MAX) + 1) {
            Err(value)
        } else if value < 0 {
            Ok(Self::with_negative(-value as u32, 1).unwrap())
        } else {
            Ok(Self::with_non_negative(value as u32, 1).unwrap())
        }
    }
}

impl TryFrom<i128> for Fraction {
    type Error = i128;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value > u32::MAX.into() || value < -(i128::from(u32::MAX) + 1) {
            Err(value)
        } else if value < 0 {
            Ok(Self::with_negative(-value as u32, 1).unwrap())
        } else {
            Ok(Self::with_non_negative(value as u32, 1).unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_primitive() {
        assert_eq!(
            Fraction::from(20_u32),
            Fraction::with_non_negative(20, 1).unwrap()
        );
        assert_eq!(
            Fraction::from(-20_i32),
            Fraction::with_negative(20, 1).unwrap()
        );
        assert_eq!(
            Fraction::from(0_i32),
            Fraction::with_non_negative(0, 1).unwrap()
        );
        assert_eq!(
            Fraction::from(0_i8),
            Fraction::with_non_negative(0, 1).unwrap()
        );
        assert_eq!(
            Fraction::from(-128_i8),
            Fraction::with_negative(128, 1).unwrap()
        );
        assert_eq!(
            Fraction::try_from(u64::from(u32::MAX)),
            Ok(Fraction::with_non_negative(u32::MAX, 1).unwrap())
        );
        assert_eq!(Fraction::try_from(u64::MAX), Err(u64::MAX));
        assert_eq!(
            Fraction::try_from(-128_i64),
            Ok(Fraction::with_negative(128, 1).unwrap())
        );
    }
}
