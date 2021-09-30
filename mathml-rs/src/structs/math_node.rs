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
    pub fn shift_indices(&mut self, shift: i32) -> &Self {
        match self {
            MathNode::Root(root) => {
                for i in 0..root.children.len() {
                    root.children[i] = ((root.children[i] as i32) + shift) as usize;
                }
                if let Some(parent) = root.parent {
                    root.parent = Some(((parent as i32) + shift) as usize);
                }
            }
            MathNode::Apply(apply) => {
                for i in 0..apply.children.len() {
                    apply.children[i] = ((apply.children[i] as i32) + shift) as usize;
                }
                if let Some(operator) = apply.operator {
                    apply.operator = Some(((operator as i32) + shift) as usize);
                }
                for i in 0..apply.operands.len() {
                    apply.operands[i] = ((apply.operands[i] as i32) + shift) as usize;
                }
                if let Some(parent) = apply.parent {
                    apply.parent = Some(((parent as i32) + shift) as usize);
                }
            }
            MathNode::Ci(ci) => {
                if let Some(parent) = ci.parent {
                    ci.parent = Some(((parent as i32) + shift) as usize);
                }
            }
            MathNode::Cn(cn) => {
                if let Some(parent) = cn.parent {
                    cn.parent = Some(((parent as i32) + shift) as usize);
                }
            }
            MathNode::Op(op) => {
                if let Some(parent) = op.parent {
                    op.parent = Some(((parent as i32) + shift) as usize);
                }
            }
            MathNode::Constant(constant) => {
                if let Some(parent) = constant.parent {
                    constant.parent = Some(((parent as i32) + shift) as usize);
                }
            }
            MathNode::Lambda(lambda) => {
                for i in 0..lambda.children.len() {
                    lambda.children[i] = ((lambda.children[i] as i32) + shift) as usize;
                }
                if let Some(expr) = lambda.expr {
                    lambda.expr = Some(((expr as i32) + shift) as usize);
                }
                for i in 0..lambda.bindings.len() {
                    lambda.bindings[i] = ((lambda.bindings[i] as i32) + shift) as usize;
                }
                if let Some(parent) = lambda.parent {
                    lambda.parent = Some(((parent as i32) + shift) as usize);
                }
            }
            MathNode::BVar(bvar) => {
                for i in 0..bvar.children.len() {
                    bvar.children[i] = ((bvar.children[i] as i32) + shift) as usize;
                }
                if let Some(parent) = bvar.parent {
                    bvar.parent = Some(((parent as i32) + shift) as usize);
                }
            }
            MathNode::Piecewise(piecewise) => {
                for i in 0..piecewise.children.len() {
                    piecewise.children[i] = ((piecewise.children[i] as i32) + shift) as usize;
                }
                if let Some(otherwise) = piecewise.otherwise {
                    piecewise.otherwise = Some(((otherwise as i32) + shift) as usize);
                }
                for i in 0..piecewise.pieces.len() {
                    piecewise.pieces[i] = ((piecewise.pieces[i] as i32) + shift) as usize;
                }
                if let Some(parent) = piecewise.parent {
                    piecewise.parent = Some(((parent as i32) + shift) as usize);
                }
            }
            MathNode::Piece(piece) => {
                for i in 0..piece.children.len() {
                    piece.children[i] = ((piece.children[i] as i32) + shift) as usize;
                }
                if let Some(expr) = piece.expr {
                    piece.expr = Some(((expr as i32) + shift) as usize);
                }
                if let Some(condition) = piece.condition {
                    piece.condition = Some(((condition as i32) + shift) as usize);
                }
                if let Some(parent) = piece.parent {
                    piece.parent = Some(((parent as i32) + shift) as usize);
                }
            }
            MathNode::Otherwise(otherwise) => {
                for i in 0..otherwise.children.len() {
                    otherwise.children[i] = ((otherwise.children[i] as i32) + shift) as usize;
                }
                if let Some(expr) = otherwise.expr {
                    otherwise.expr = Some(((expr as i32) + shift) as usize);
                }
                if let Some(parent) = otherwise.parent {
                    otherwise.parent = Some(((parent as i32) + shift) as usize);
                }
            }
        }
        self
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
