use super::math_node::{MathNodeType, NodeIndex};
use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct Lambda {
    pub children: Vec<NodeIndex>,
    pub bindings: Vec<NodeIndex>,
    pub expr: Option<NodeIndex>,
    pub parent: Option<NodeIndex>,
}

impl Lambda {
    pub fn index(&mut self, tag_type: MathNodeType, location: NodeIndex) {
        match tag_type {
            MathNodeType::Op | MathNodeType::Apply | MathNodeType::Lambda => {
                if self.expr == None {
                    self.expr = Some(location);
                } else {
                    panic!("Can't have two operators in an apply node!");
                }
            }
            MathNodeType::BVar => {
                self.bindings.push(location);
            }
            MathNodeType::Root | MathNodeType::Ci | MathNodeType::Cn => {}
        }
    }
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
