pub type NodeIndex = usize;
use super::apply::Apply;
use super::ci::Ci;
use super::cn::Cn;
use super::op::{Op, OpNode};
use super::root::Root;
use std::fmt;

#[derive(Debug, Clone)]
pub enum MathNode {
    Apply(Apply),
    Op(OpNode),
    Root(Root),
    Ci(Ci),
    Cn(Cn),
    //Lambda(Lambda),
}

impl MathNode {
    pub fn new_op(op: Op) -> Self {
        MathNode::Op(OpNode {
            op: Some(op),
            parent: None,
        })
    }
}

impl Default for MathNode {
    fn default() -> Self {
        MathNode::Root(Root::default())
    }
}

impl fmt::Display for MathNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathNode::Apply(apply) => write!(f, "Apply: {}", apply),
            MathNode::Root(root) => write!(f, "Root: {}", root),
            MathNode::Ci(ci) => write!(f, "Ci: {}", ci),
            MathNode::Op(opnode) => write!(f, "Op: {}", opnode),
            MathNode::Cn(cn) => write!(f, "Cn: {}", cn),
        }
    }
}

pub enum MathNodeType {
    Apply,
    Op,
    Root,
    Ci,
    Cn,
}
