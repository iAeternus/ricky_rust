use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::frac::{gcd, Fraction, FractionSign};

impl Add<Fraction> for Fraction {
    type Output = Self;

    fn add(self, rhs: Fraction) -> Self::Output {
        match (self.sign(), rhs.sign()) {
            (FractionSign::NonNegative, FractionSign::NonNegative)
            | (FractionSign::Negative, FractionSign::Negative) => {
                let numer = u64::from(self.numer()) * u64::from(rhs.denom())
                    + u64::from(self.denom()) * u64::from(rhs.numer());
                let denom = u64::from(self.denom()) * u64::from(rhs.denom());
                let gcd = gcd(numer, denom);
                let numer = u32::try_from(numer / gcd).expect("add overflow");
                let denom = u32::try_from(denom / gcd).expect("add overflow");
                Self::new(numer, denom, self.sign()).unwrap()
            }
            (FractionSign::NonNegative, FractionSign::Negative)
            | (FractionSign::Negative, FractionSign::NonNegative) => {
                let numer_part1 = u64::from(self.numer()) * u64::from(rhs.denom());
                let numer_part2 = u64::from(self.denom()) * u64::from(rhs.numer());
                let denom = u64::from(self.denom()) * u64::from(rhs.denom());
                let (numer, sign) = if numer_part1 >= numer_part2 {
                    (numer_part1 - numer_part2, FractionSign::NonNegative)
                } else {
                    (numer_part2 - numer_part1, FractionSign::Negative)
                };
                let gcd = gcd(numer, denom);
                let numer = u32::try_from(numer / gcd).expect("subtract overflow");
                let denom = u32::try_from(denom / gcd).expect("subtract overflow");
                Self::new(numer, denom, sign).unwrap()
            }
        }
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.numer(), self.denom(), -self.sign()).unwrap()
    }
}

impl Sub<Fraction> for Fraction {
    type Output = Self;

    fn sub(self, rhs: Fraction) -> Self::Output {
        self.add(rhs.neg())
    }
}

impl Mul<Fraction> for Fraction {
    type Output = Self;

    fn mul(self, rhs: Fraction) -> Self::Output {
        let numer = u64::from(self.numer()) * u64::from(rhs.numer());
        let denom = u64::from(self.denom()) * u64::from(rhs.denom());
        let gcd = gcd(numer, denom);
        let numer = u32::try_from(numer / gcd).expect("multiply overflow");
        let denom = u32::try_from(denom / gcd).expect("multiply overflow");
        let sign = FractionSign::from(self.sign() as u8 ^ rhs.sign() as u8);
        Self::new(numer, denom, sign).unwrap()
    }
}

impl Div<Fraction> for Fraction {
    type Output = Self; 

    fn div(self, rhs: Fraction) -> Self::Output {
        if rhs == 0 {
            panic!("divide by zero");
        } else {
            self * Self::new(self.denom(), self.numer(), self.sign()).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_add() {
        assert_eq!(
            Fraction::with_non_negative(1, 2).unwrap() + Fraction::with_non_negative(1, 2).unwrap(),
            Fraction::new(1, 1, FractionSign::NonNegative).unwrap()
        );
        assert_eq!(
            Fraction::with_non_negative(1, 2).unwrap() + Fraction::with_negative(1, 2).unwrap(),
            Fraction::new(0, 1, FractionSign::NonNegative).unwrap()
        );
        assert_eq!(
            Fraction::with_non_negative(1, 2).unwrap() + Fraction::with_negative(1, 3).unwrap(),
            Fraction::new(1, 6, FractionSign::NonNegative).unwrap()
        );
        assert_eq!(
            Fraction::with_negative(1, 2).unwrap() + Fraction::with_negative(1, 3).unwrap(),
            Fraction::new(5, 6, FractionSign::Negative).unwrap()
        );
        assert_eq!(
            Fraction::with_non_negative(0, 1).unwrap() + Fraction::with_negative(1, 2).unwrap(),
            Fraction::new(1, 2, FractionSign::Negative).unwrap()
        );
        assert_eq!(
            Fraction::with_negative(1, 6).unwrap() + Fraction::with_negative(1, 2).unwrap(),
            Fraction::new(2, 3, FractionSign::Negative).unwrap()
        );
        assert_eq!(
            Fraction::with_non_negative(1, 3).unwrap() + Fraction::with_negative(1, 2).unwrap(),
            Fraction::new(1, 6, FractionSign::Negative).unwrap()
        );
        assert_eq!(
            Fraction::with_non_negative(1, 2).unwrap() + 1.into(),
            Fraction::with_non_negative(3, 2).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_fraction_add_with_overflow() {
        let f = Fraction::with_non_negative(u32::MAX - 1, u32::MAX).unwrap();
        let _ = f + f;
    }
}
