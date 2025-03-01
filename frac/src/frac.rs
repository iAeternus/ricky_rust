/// 分数
#[derive(Debug, PartialEq, Eq)]
pub struct Fraction {
    numer: u32,
    denom: u32,
    sign: FractionSign,
}

/// 符号
#[derive(Debug, PartialEq, Eq)]
pub enum FractionSign {
    NonNegative = 0,
    Negative = 1,
}

impl Fraction {
    pub fn new(numer: u32, denom: u32, sign: FractionSign) -> Option<Self> {
        if denom == 0 {
            return None;
        }
        let gcd = gcd(numer, denom);
        Some(Self {
            numer: numer / gcd,
            denom: denom / gcd,
            sign,
        })
    }

    pub fn with_non_negative(numer: u32, denom: u32) -> Option<Self> {
        Self::new(numer, denom, FractionSign::NonNegative)
    }

    pub fn with_negative(numer: u32, denom: u32) -> Option<Self> {
        Self::new(numer, denom, FractionSign::Negative)
    }
}

/// 计算两个数的最大公约数
fn gcd(mut a: u32, mut b: u32) -> u32 {
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
