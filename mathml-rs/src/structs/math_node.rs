pub type NodeIndex = usize;
use crate::{
    Apply, BVar, Ci, Cn, Constant, ConstantNode, Lambda, Op, OpNode, Otherwise, Piece, Piecewise,
    Root,
};

use std::fmt;

#[derive(Debug, Clone)]
pub enum MathNode {
    Apply(Apply),
    Op(OpNode),
    Constant(ConstantNode),
    Root(Root),
    Ci(Ci),
    Cn(Cn),
    Lambda(Lambda),
    BVar(BVar),
    Piecewise(Piecewise),
    Piece(Piece),
    Otherwise(Otherwise),
}

impl MathNode {
    pub fn new_op(op: Op) -> Self {
        MathNode::Op(OpNode {
            op: Some(op),
            parent: None,
        })
    }
    pub fn new_constant(constant: Constant) -> Self {
        MathNode::Constant(ConstantNode {
            constant: Some(constant),
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
            MathNode::Lambda(lambda) => write!(f, "Lambda: {}", lambda),
            MathNode::BVar(bvar) => write!(f, "BVar: {}", bvar),
            MathNode::Piecewise(piecewise) => write!(f, "Piecewise: {}", piecewise),
            MathNode::Piece(piece) => write!(f, "Piece: {}", piece),
            MathNode::Otherwise(otherwise) => write!(f, "Otherwise: {}", otherwise),
            MathNode::Constant(constantnode) => write!(f, "Constant: {}", constantnode),
        }
    }
}

pub enum MathNodeType {
    Apply,
    Op,
    Root,
    Ci,
    Cn,
    Lambda,
    BVar,
    Piecewise,
    Piece,
    Otherwise,
    Constant,
}

impl fmt::Display for MathNodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathNodeType::Apply => write!(f, "Apply"),
            MathNodeType::Root => write!(f, "Root"),
            MathNodeType::Ci => write!(f, "Ci"),
            MathNodeType::Op => write!(f, "Op"),
            MathNodeType::Constant => write!(f, "Constant"),
            MathNodeType::Cn => write!(f, "Cn"),
            MathNodeType::Lambda => write!(f, "Lambda"),
            MathNodeType::BVar => write!(f, "BVar"),
            MathNodeType::Piecewise => write!(f, "Piecewise"),
            MathNodeType::Piece => write!(f, "Piece"),
            MathNodeType::Otherwise => write!(f, "Otherwise"),
        }
    }
}
