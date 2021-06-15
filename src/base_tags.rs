use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Eq, PartialEq, Deserialize, Clone)]
#[serde(rename = "math", rename_all = "camelCase")]
pub enum MathNode {
    Op(BuiltinOp),
    Apply(Vec<MathNode>),
    Text(String),
    #[serde(rename = "math")]
    Root(Vec<MathNode>),
    Ci(Vec<MathNode>),
    //Csymbol {
    //definition_url: String,
    //encoding: Option<String>,
    //children: Vec<MathNode>,
    //},
    //Cn {
    ////num_type: numbers::NumType,
    //num_type: String,
    ////base: u32,
    ////definition_url: Option<String>,
    ////encoding: Option<String>,
    ////attributes: Option<HashMap<String, String>>,
    //},
    //Comment(String),
    //PI(String, Option<String>),
    //Math(String, Box<MathNode>),
}

#[derive(Deserialize, Debug, Serialize, Eq, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum BuiltinOp {
    factorial,
    minus,
    abs,
    conjugate,
    arg,
    real,
    imaginary,
    floor,
    ceiling,
    not,
    inverse,
    ident,
    domain,
    codomain,
    image,
    sin,
    cos,
    tan,
    sec,
    csc,
    cot,
    sinh,
    cosh,
    tanh,
    sech,
    csch,
    coth,
    arcsin,
    arccos,
    arctan,
    arccosh,
    arccot,
    arccoth,
    arccsc,
    arccsch,
    arcsec,
    arcsech,
    arcsinh,
    arctanh,
    exp,
    ln,
    log,
    determinant,
    transpose,
    divergence,
    grad,
    curl,
    laplacian,
    card,
    quotient,
    divide,
    power,
    rem,
    implies,
    equivalent,
    approx,
    setdiff,
    vectorproduct,
    scalarproduct,
    outerproduct,
    plus,
    times,
    max,
    min,
    gcd,
    lcm,
    mean,
    sdev,
    variance,
    median,
    mode,
    and,
    or,
    xor,
    selector,
    union,
    intersect,
    cartesianproduct,
    compose,
    r#fn,
    int,
    sum,
    product,
    diff,
    partialdiff,
    forall,
    exists,
    eq,
    neq,
    gt,
    lt,
    geq,
    leq,
    root,
}