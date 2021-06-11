mod base_tags;
pub use base_tags::MathNode;

use quick_xml::de::{from_str, DeError};
use serde::{Deserialize, Serialize};

pub fn parse(doc: &str) -> Result<MathNode, quick_xml::DeError> {
    let raw_model = quick_xml::de::from_str(doc);
    raw_model
}
