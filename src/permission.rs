
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Permission {
    Administrator,
    StdIo
}

impl Permission {
    pub fn from_string(permission: &str) -> Self {
        match permission {
            "Administrator" => Self::Administrator,
            "StdIo" => Self::StdIo,
            _ => panic!("Permission not found.")
        }
    }
}