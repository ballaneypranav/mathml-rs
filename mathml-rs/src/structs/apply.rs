use super::math_node::NodeIndex;
use std::fmt;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Apply {
    pub children: Vec<NodeIndex>,
    pub parent: Option<NodeIndex>,
}

impl fmt::Display for Apply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Children: {:?}, Parent: {:?}",
            self.children, self.parent
        )
    }
}
