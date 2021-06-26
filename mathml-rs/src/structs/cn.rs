use super::math_node::NodeIndex;
use super::numbers::{NumType, Number};
use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct Cn {
    pub r#type: Option<NumType>,
    pub value: Option<Number>,
    pub parent: Option<NodeIndex>,
}

impl fmt::Display for Cn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Some(Number::Integer(i)) => write!(f, "integer: {}", i),
            Some(Number::Real(r)) => write!(f, "real: {}", r),
            Some(Number::Rational(n, d)) => write!(f, "rational: {} / {}", n, d),
            Some(Number::ComplexPolar(a, b)) | Some(Number::ComplexCartesian(a, b)) => {
                write!(f, "complex: {}, {}", a, b)
            }
            Some(Number::Constant(s)) => write!(f, "complex: {}", s),
            Some(Number::ENotation(a, b)) => write!(f, "exp: {} ^ {}", a, b),
            None => write!(f, "type is None"),
        }
    }
}

//impl PartialEq for Cn {
//fn eq(&self, other: &Self) -> bool {
//match (self, other) {
//(Real(r), Real(r2)) => approx::abs_diff_eq!(r, r2),
//(Integer(r1), Integer(r2)) => r1 == r2,
//(Rational(a, b), Rational(c, d)) => (a == c) && (b == d),
//(ComplexPolar(a, b), ComplexPolar(c, d))
//| (ComplexCartesian(a, b), ComplexCartesian(c, d)) => {
//approx::abs_diff_eq!(a, c) && approx::abs_diff_eq!(b, d)
//}
//(Constant(a), Constant(b)) => a == b,
//(ENotation(a, b), ENotation(c, d)) => a == c && b == d,
//_ => false,
//}
//}
//}
