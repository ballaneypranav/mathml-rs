use super::math_node::{MathNode, MathNodeType, NodeIndex};
use super::op::Op;
use std::fmt;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Apply {
    pub children: Vec<NodeIndex>,
    pub operator: Option<NodeIndex>,
    pub operands: Vec<NodeIndex>,
    pub parent: Option<NodeIndex>,
}

impl Apply {
    pub fn index(&mut self, tag_type: MathNodeType, location: NodeIndex) {
        match tag_type {
            MathNodeType::Op => {
                if self.operator == None {
                    self.operator = Some(location);
                } else {
                    panic!("Can't have two operators in an apply node!");
                }
            }
            MathNodeType::Apply | MathNodeType::Ci | MathNodeType::Cn => {
                self.operands.push(location);
            }
            MathNodeType::Root => {}
        }
    }

    pub fn get_op(&self, nodes: &Vec<MathNode>) -> Result<Op, &'static str> {
        let operator_idx = self.operator.expect("No operator found!");
        if let MathNode::Op(opnode) = &nodes[operator_idx] {
            return Ok(opnode.op.clone().unwrap());
        }
        Err("No operator found!")
    }
}

impl fmt::Display for Apply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Operator: {:?}, Operands: {:?}, Children: {:?}, Parent: {:?}",
            self.operator, self.operands, self.children, self.parent
        )
    }
}
