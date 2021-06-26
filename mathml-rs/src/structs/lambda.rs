use super::math_node::NodeIndex;
use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct Lambda {
    children: Vec<NodeIndex>,
    bindings: Vec<NodeIndex>,
    expr: Option<NodeIndex>,
    parent: Option<NodeIndex>,
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Bindings: {:?}, Expr: {:?}, Children: {:?}, Parent: {:?}",
            self.bindings, self.expr, self.children, self.parent
        )
    }
}
