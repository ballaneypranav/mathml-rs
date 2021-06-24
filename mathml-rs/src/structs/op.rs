use super::math_node::NodeIndex;
use approx;
use std::fmt;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct OpNode {
    pub op: Option<Op>,
    pub parent: Option<NodeIndex>,
}

impl fmt::Display for OpNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "op: {:?}, parent: {:?}", self.op, self.parent)
    }
}

#[derive(Debug, Clone)]
pub enum NumType {
    Real(f64),
    Integer(i64),
    Rational(i64, i64),
    ComplexCartesian(f64, f64),
    ComplexPolar(f64, f64),
    Constant(String),
    ENotation(f64, i64),
}

impl Eq for NumType {}
impl PartialEq for NumType {
    fn eq(&self, other: &Self) -> bool {
        use NumType::*;
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Op {
    Factorial,
    Minus,
    Abs,
    Conjugate,
    Arg,
    Real,
    Imaginary,
    Floor,
    Ceiling,
    Not,
    Inverse,
    Ident,
    Domain,
    Codomain,
    Image,
    Sin,
    Cos,
    Tan,
    Sec,
    Csc,
    Cot,
    Sinh,
    Cosh,
    Tanh,
    Sech,
    Csch,
    Coth,
    Arcsin,
    Arccos,
    Arctan,
    Arccosh,
    Arccot,
    Arccoth,
    Arccsc,
    Arccsch,
    Arcsec,
    Arcsech,
    Arcsinh,
    Arctanh,
    Exp,
    Ln,
    Log,
    Determinant,
    Transpose,
    Divergence,
    Grad,
    Curl,
    Laplacian,
    Card,
    Quotient,
    Divide,
    Power,
    Rem,
    Implies,
    Equivalent,
    Approx,
    Setdiff,
    Vectorproduct,
    Scalarproduct,
    Outerproduct,
    Plus,
    Times,
    Max,
    Min,
    Gcd,
    Lcm,
    Mean,
    Sdev,
    Variance,
    Median,
    Mode,
    And,
    Or,
    Xor,
    Selector,
    Union,
    Intersect,
    Cartesianproduct,
    Compose,
    r#Fn,
    Int,
    Sum,
    Product,
    Diff,
    Partialdiff,
    Forall,
    Exists,
    Eq,
    Neq,
    Gt,
    Lt,
    Geq,
    Leq,
    Root,
}
