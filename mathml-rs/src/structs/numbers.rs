use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum NumType {
    Real,
    Integer,
    Rational,
    ComplexCartesian,
    ComplexPolar,
    Constant,
    ENotation,
}

impl FromStr for NumType {
    type Err = ();

    fn from_str(s: &str) -> Result<NumType, ()> {
        match s {
            "integer" => Ok(NumType::Integer),
            "real" | "double" => Ok(NumType::Real),
            "e-notation" => Ok(NumType::ENotation),
            "rational" => Ok(NumType::Rational),
            "complex-cartesian" => Ok(NumType::ComplexCartesian),
            "complex-polar" => Ok(NumType::ComplexPolar),
            "constant" => Ok(NumType::Constant),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Number {
    Real(f64),
    Integer(i32),
    Rational(i64, i64),
    ComplexCartesian(f64, f64),
    ComplexPolar(f64, f64),
    Constant(String),
    ENotation(f64, i64),
}

impl Eq for Number {}
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        use Number::*;
        match (self, other) {
            (Real(r), Real(r2)) => approx::abs_diff_eq!(r, r2),
            (Integer(r1), Integer(r2)) => r1 == r2,
            (Rational(a, b), Rational(c, d)) => (a == c) && (b == d),
            (ComplexPolar(a, b), ComplexPolar(c, d))
            | (ComplexCartesian(a, b), ComplexCartesian(c, d)) => {
                approx::abs_diff_eq!(a, c) && approx::abs_diff_eq!(b, d)
            }
            (Constant(a), Constant(b)) => a == b,
            (ENotation(a, b), ENotation(c, d)) => a == c && b == d,
            _ => false,
        }
    }
}
