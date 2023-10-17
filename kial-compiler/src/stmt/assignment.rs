use crate::env::Env;
use crate::expr::binding_usage::BindingUsage;
use crate::expr::Expr;
use crate::val::Val;
use crate::{utils, val};

#[derive(Debug, PartialEq)]
pub(crate) struct Assignment {
    name: String,
    val: Expr,
}

impl Assignment {
    pub(crate) fn new(name: &str, val: Expr) -> Self {
        Assignment {
            name: name.to_string(),
            val,
        }
    }

    pub(crate) fn parse(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let (s, ident) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("=", s)?;
        let (s, _) = utils::extract_whitespace(s);
        let (s, expr) = Expr::parse(s)?;
        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag(";", s)?;
        let (s, _) = utils::extract_whitespace(s);

        Ok((s, Assignment::new(ident, expr)))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Val, String> {
        let val = self.val.eval(env)?;
        env.get_binding(self.name.as_str())?;
        env.store_binding(self.name.clone(), val);
        Ok(Val::Unit)
    }
}

#[cfg(test)]
mod tests {
    use crate::env::Env;
    use crate::expr::binding_usage::BindingUsage;
    use crate::expr::operation::Operation;
    use crate::expr::{Expr, Op};
    use crate::stmt::Assignment;
    use crate::val::Val;

    #[test]
    fn parse_simple_assignment() {
        assert_eq!(
            Assignment::parse("a = a * b;"),
            Ok((
                "",
                Assignment::new(
                    "a",
                    Expr::Operation(Operation::new(
                        Expr::BindingUsage(BindingUsage::new("a")),
                        Expr::BindingUsage(BindingUsage::new("b")),
                        Op::Mul
                    ))
                )
            ))
        )
    }

    #[test]
    fn parse_assignment_whitespace() {
        assert_eq!(
            Assignment::parse("      \r\n    a\t     =      \t a\r      \n/\t    b   \n\n\n\n;\n"),
            Ok((
                "",
                Assignment::new(
                    "a",
                    Expr::Operation(Operation::new(
                        Expr::BindingUsage(BindingUsage::new("a")),
                        Expr::BindingUsage(BindingUsage::new("b")),
                        Op::Div
                    ))
                )
            ))
        )
    }

    #[test]
    fn eval_simple_self_multiplication() {
        let mut env = Env::default();
        env.store_binding("a".to_string(), Val::Number(10));
        env.store_binding("b".to_string(), Val::Number(3));

        assert_eq!(
            Assignment::new(
                "a",
                Expr::Operation(Operation::new(
                    Expr::BindingUsage(BindingUsage::new("a")),
                    Expr::BindingUsage(BindingUsage::new("b")),
                    Op::Mul
                ))
            )
            .eval(&mut env),
            Ok(Val::Unit)
        );

        assert_eq!(env.get_binding("a"), Ok(Val::Number(30)));
    }

    #[test]
    fn eval_var_not_defined() {
        let mut env = Env::default();
        env.store_binding("b".to_string(), Val::Number(3));

        assert_eq!(
            Assignment::new(
                "a",
                Expr::Operation(Operation::new(
                    Expr::BindingUsage(BindingUsage::new("a")),
                    Expr::BindingUsage(BindingUsage::new("b")),
                    Op::Mul
                ))
            )
            .eval(&mut env),
            Err("Binding does not exist: a".to_string())
        );
    }

    #[test]
    fn eval_self_assignment() {
        let mut env = Env::default();
        env.store_binding("a".to_string(), Val::Number(15));

        assert_eq!(
            Assignment::new("a", Expr::BindingUsage(BindingUsage::new("a"))).eval(&mut env),
            Ok(Val::Unit)
        );
        assert_eq!(env.get_binding("a"), Ok(Val::Number(15)));
    }
}
