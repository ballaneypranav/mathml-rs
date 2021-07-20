use super::math_node::{MathNodeType, NodeIndex};
use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct Piecewise {
    pub children: Vec<NodeIndex>,
    pub pieces: Vec<NodeIndex>,
    pub otherwise: Option<NodeIndex>,
    pub parent: Option<NodeIndex>,
}

impl Piecewise {
    pub fn index(&mut self, tag_type: MathNodeType, location: NodeIndex) {
        match tag_type {
            MathNodeType::Piece => {
                self.pieces.push(location);
            }
            MathNodeType::Otherwise => {
                if self.otherwise == None {
                    self.otherwise = Some(location);
                } else {
                    panic!("Can't have multiple otherwise expressions in an piecewise function!");
                }
            }
            MathNodeType::Op
            | MathNodeType::Ci
            | MathNodeType::Cn
            | MathNodeType::Root
            | MathNodeType::BVar
            | MathNodeType::Apply
            | MathNodeType::Lambda
            | MathNodeType::Piecewise
            | MathNodeType::Constant => {
                let error = format!("Can't have {} in a piecewise function!", tag_type);
                panic!(error);
            }
        }
    }
}

impl fmt::Display for Piecewise {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Pieces: {:?}, Otherwise: {:?}, Children: {:?}, Parent: {:?}",
            self.pieces, self.otherwise, self.children, self.parent
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct Piece {
    pub children: Vec<NodeIndex>,
    pub expr: Option<NodeIndex>,
    pub condition: Option<NodeIndex>,
    pub parent: Option<NodeIndex>,
}

impl Piece {
    pub fn index(&mut self, _tag_type: MathNodeType, location: NodeIndex) {
        if self.children.len() == 1 {
            self.expr = Some(location);
        } else if self.children.len() == 2 {
            self.condition = Some(location);
        } else {
            panic!("A piece in a piecewise function can have only two children.");
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Expr: {:?}, Condition: {:?}, Children: {:?}, Parent: {:?}",
            self.expr, self.condition, self.children, self.parent
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct Otherwise {
    pub children: Vec<NodeIndex>,
    pub expr: Option<NodeIndex>,
    pub parent: Option<NodeIndex>,
}

impl Otherwise {
    pub fn index(&mut self, tag_type: MathNodeType, location: NodeIndex) {
        match tag_type {
            MathNodeType::Apply
            | MathNodeType::Lambda
            | MathNodeType::Ci
            | MathNodeType::Cn
            | MathNodeType::Piecewise
            | MathNodeType::Constant => {
                if self.expr == None {
                    self.expr = Some(location);
                } else {
                    panic!("Can't have two children in an \"otherwise\" branch!");
                }
            }
            MathNodeType::Root
            | MathNodeType::Op
            | MathNodeType::Otherwise
            | MathNodeType::BVar
            | MathNodeType::Piece => {
                let error = format!("Can't have {} in a \"otherwise\" branch!", tag_type);
                panic!(error);
            }
        }
    }
}

impl fmt::Display for Otherwise {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Expr: {:?}, Children: {:?}, Parent: {:?}",
            self.expr, self.children, self.parent
        )
    }
}
