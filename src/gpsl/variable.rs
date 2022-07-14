use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Variable {
    Number { value: usize },
    Text { value: String },
    Return { value: Box<Variable> },
    None {},
}
