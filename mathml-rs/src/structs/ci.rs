use super::math_node::NodeIndex;
use std::fmt;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Ci {
    pub name: Option<String>,
    pub parent: Option<NodeIndex>,
}

impl Ci {
    pub fn with_name(s: String) -> Self {
        Ci {
            name: Some(s),
            parent: None,
        }
    }
}

impl fmt::Display for Ci {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "text: {:?}, parent: {:?}", self.name, self.parent)
    }
}
