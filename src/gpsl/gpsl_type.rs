use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GPSLType {
    pub type_str: String,
    pub child: Vec<GPSLType>,
}
