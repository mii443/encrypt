
#[derive(Clone, PartialEq, Debug)]
pub enum Variable {
    Number {
        value: usize,
    },
    Text {
        value: String,
    },
    Return {
        value: Box<Variable>
    },
    None {}
}
