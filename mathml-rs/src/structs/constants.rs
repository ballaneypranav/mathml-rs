use super::math_node::NodeIndex;
use std::fmt;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct ConstantNode {
    pub constant: Option<Constant>,
    pub parent: Option<NodeIndex>,
}

impl fmt::Display for ConstantNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "constant: {:?}, parent: {:?}",
            self.constant, self.parent
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Constant {
    ImaginaryI,
    True,
    False,
    Pi,
    EulerGamma,
    Infinity,
}
