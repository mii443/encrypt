use crate::{variable::Variable, permission::Permission};

#[derive(PartialEq)]
pub enum ExternalFuncStatus {
    SUCCESS,
    NOTFOUND,
    ERROR,
    REJECTED,
}

pub struct ExternalFuncReturn {
    pub status: ExternalFuncStatus,
    pub value: Option<Variable>
}

#[allow(dead_code)]
pub const STD_FUNC: fn(String, Vec<Variable>, Vec<Permission>, Vec<Permission>) -> ExternalFuncReturn = |name, args, accept, reject| {
    let name = name.as_str();
    match name {
        "println" => {
            if accept.contains(&Permission::StdIo) && !reject.contains(&Permission::StdIo) {
                match &args[0] {
                    Variable::Text { value } => println!("{}", value),
                    Variable::Number { value } => println!("{}", value),
                    _ => {}
                }
                ExternalFuncReturn {
                    status: ExternalFuncStatus::SUCCESS,
                    value: None
                }
            } else {
                ExternalFuncReturn {
                    status: ExternalFuncStatus::REJECTED,
                    value: None
                }
            }
        }
        "print" => {
            if accept.contains(&Permission::StdIo) && !reject.contains(&Permission::StdIo) {
                match &args[0] {
                    Variable::Text { value } => print!("{}", value),
                    Variable::Number { value } => print!("{}", value),
                    _ => {}
                }
                ExternalFuncReturn {
                    status: ExternalFuncStatus::SUCCESS,
                    value: None
                }
            } else {
                ExternalFuncReturn {
                    status: ExternalFuncStatus::REJECTED,
                    value: None
                }
            }
        }
        _ => {
            ExternalFuncReturn {
                status: ExternalFuncStatus::NOTFOUND,
                value: None
            }
        }
    }
};
