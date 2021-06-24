use super::math_node::NodeIndex;
use std::fmt;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Cn {
    pub integer: Option<i32>,
    pub real: Option<f32>,
    pub double: Option<f64>,
    pub r#type: Option<String>,
    pub parent: Option<NodeIndex>,
}

impl fmt::Display for Cn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.r#type.as_deref() {
            Some("integer") => write!(f, "integer: {}", self.integer.unwrap()),
            Some("real") => write!(f, "integer: {}", self.real.unwrap()),
            Some("double") => write!(f, "integer: {}", self.double.unwrap()),
            Some(s) => write!(f, "invalid type: {}", s),
            None => write!(f, "type is None"),
        }
    }
}
