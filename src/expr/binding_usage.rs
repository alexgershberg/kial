use crate::env::Env;
use crate::utils;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub(crate) struct BindingUsage {
    pub name: String,
}

impl BindingUsage {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = utils::extract_ident(s)?;
        Ok((
            s,
            Self {
                name: name.to_string(),
            },
        ))
    }

    pub fn eval(self, env: &Env) -> Result<Val, String> {
        env.get_binding(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::val::Val;

    #[test]
    fn eval_binding_not_found() {
        let env = Env::default();
        assert_eq!(
            BindingUsage {
                name: "bar".to_string(),
            }
            .eval(&env),
            Err("Binding does not exist: bar".to_string())
        )
    }

    #[test]
    fn eval_binding_usage() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(10));
        assert_eq!(
            BindingUsage {
                name: "foo".to_string(),
            }
            .eval(&mut env),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string()
                }
            ))
        )
    }
}
