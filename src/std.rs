use crate::variable::Variable;
use crate::gpsl::*;

#[allow(dead_code)]
pub const STD_FUNC: fn(String, Vec<Variable>) -> ExternalFuncReturn = |name, args| {
    let name = name.as_str();
    match name {
        "println" => {
            match &args[0] {
                Variable::Text { value } => println!("{}", value),
                Variable::Number { value } => println!("{}", value),
                _ => {}
            }
            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: None
            }
        }
        "print" => {
            match &args[0] {
                Variable::Text { value } => print!("{}", value),
                Variable::Number { value } => print!("{}", value),
                _ => {}
            }
            ExternalFuncReturn {
                status: ExternalFuncStatus::SUCCESS,
                value: None
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
