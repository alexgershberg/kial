use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Number(i32),
    Unit,
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let repr = match self {
            Val::Number(num) => {
                format!("{num}")
            }
            Val::Unit => "()".to_string(),
        };

        write!(f, "{}", repr)
    }
}
