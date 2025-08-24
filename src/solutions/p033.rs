use std::ops::Mul;

use crate::solutions::Solution;

pub struct Problem033;

impl Problem033 {
    pub fn new() -> Self {
        Self
    }
}

struct Fraction {
    numerator: u32,
    denominator: u32,
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
        }
    }
}

impl Fraction {
    fn new(numerator: u32, denominator: u32) -> Self {
        Self { numerator, denominator }
    }

    fn resolve(&self) -> Self {
        let gcd = self.greatest_common_divisor(self.numerator, self.denominator);
        Self {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }

    fn greatest_common_divisor(&self, a: u32, b: u32) -> u32 {
        if b == 0 {
            a
        } else {
            self.greatest_common_divisor(b, a % b)
        }
    }

    fn curious_resolve(&self) -> bool {
        let numerator = self.numerator;
        let denominator = self.denominator;

        if numerator >= denominator {
            return false;
        }

        let n1 = numerator / 10;
        let n2 = numerator % 10;
        let d1 = denominator / 10;
        let d2 = denominator % 10;

        if n2 == 0 && d2 == 0 {
            return false;
        }

        if n1 == d1 && d2 != 0 && Fraction::new(n2, d2).resolve() == self.resolve() {
            return true;
        }
        if n1 == d2 && d1 != 0 && Fraction::new(n2, d1).resolve() == self.resolve() {
            return true;
        }
        if n2 == d1 && d2 != 0 && Fraction::new(n1, d2).resolve() == self.resolve() {
            return true;
        }
        if n2 == d2 && d1 != 0 && Fraction::new(n1, d1).resolve() == self.resolve() {
            return true;
        }

        false
    }
}

impl Solution for Problem033 {
    fn number(&self) -> u32 { 33 }

    fn title(&self) -> &'static str {
        "Digit Cancelling Fractions"
    }

    fn solve(&self) -> String {

        let mut product = Fraction::new(1, 1);

        for numerator in 10..100 {
            for denominator in 10..100 {
                let fraction = Fraction::new(numerator, denominator);
                if fraction.curious_resolve() {
                    println!("Curious fraction: {}/{}", numerator, denominator);
                    product = product * fraction;
                }
            }
        }

        product.resolve().denominator.to_string()
    }
}