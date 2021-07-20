use super::lambda::Lambda;
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
    pub fn index(&mut self, _tag_type: MathNodeType, location: NodeIndex) {
        if self.children.len() == 1 {
            self.operator = Some(location);
        } else {
            self.operands.push(location);
        }
    }

    pub fn get_op(&self, nodes: &Vec<MathNode>) -> Result<Op, &'static str> {
        let operator_idx = self.operator.expect("No operator found!");
        if let MathNode::Op(opnode) = &nodes[operator_idx] {
            return Ok(opnode.op.clone().unwrap());
        }
        Err("Not a regular mathematical operator.")
    }

    pub fn get_lambda(&self, nodes: &Vec<MathNode>) -> Result<Lambda, &'static str> {
        let operator_idx = self.operator.expect("No operator found!");
        if let MathNode::Lambda(lambda) = &nodes[operator_idx] {
            return Ok(lambda.clone());
        }
        Err("Not a lambda function.")
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
