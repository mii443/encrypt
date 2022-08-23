#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Permission {
    Administrator,
    StdIn,
    StdOut,
    FileWrite,
}

impl Permission {
    pub fn from_string(permission: &str) -> Self {
        match permission {
            "Administrator" => Self::Administrator,
            "StdIn" => Self::StdIn,
            "StdOut" => Self::StdOut,
            "FileWrite" => Self::FileWrite,
            _ => panic!("Permission not found."),
        }
    }
}
