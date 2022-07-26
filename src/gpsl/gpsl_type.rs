use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GPSLType {
    pub type_str: String,
    pub child: Vec<GPSLType>,
}

impl Display for GPSLType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_str)
    }
}

impl GPSLType {
    pub fn to_str(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.type_str);
        let children: Vec<String> = self.child.iter().map(|c| c.to_str()).collect();
        if children.len() > 0 {
            s.push_str("<");
            s.push_str(&children.join(","));
            s.push_str(">");
        }
        s
    }

    pub fn from_str(s: &str) -> Result<GPSLType, String> {
        let mut type_str = String::new();
        let mut child = Vec::new();
        let mut i = 0;
        let mut is_child = false;
        let mut is_type_str = true;
        let mut is_child_str = false;
        let mut child_str = String::new();
        for c in s.chars() {
            if is_child_str {
                if c == '>' {
                    is_child_str = false;
                    is_child = true;
                } else {
                    child_str.push(c);
                }
            } else if is_child {
                if c == '<' {
                    is_child = false;
                    is_child_str = true;
                } else {
                    child.push(GPSLType::from_str(&c.to_string())?);
                }
            } else if is_type_str {
                if c == '<' {
                    is_type_str = false;
                    is_child = true;
                } else {
                    type_str.push(c);
                }
            } else {
                return Err(format!("LOOP Invalid GPSLType: {}", s));
            }
            i += 1;
        }
        Ok(GPSLType { type_str, child })
    }

    pub fn is_correct(&self) -> bool {
        if self.type_str == "U512"
            || self.type_str == "num"
            || self.type_str == "String"
            || self.type_str == "eep"
            || self.type_str == "eep_p"
        {
            return true;
        }
        if self.child.len() == 0 {
            return false;
        }
        for c in &self.child {
            if !c.is_correct() {
                return false;
            }
        }
        true
    }
}
