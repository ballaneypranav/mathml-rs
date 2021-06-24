use super::math_node::NodeIndex;
use std::fmt;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Root {
    pub children: Vec<NodeIndex>,
    pub parent: Option<NodeIndex>,
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "children: {:?}, parent: {:?}",
            self.children, self.parent
        )
    }
}
