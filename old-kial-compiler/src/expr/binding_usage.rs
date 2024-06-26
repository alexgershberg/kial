use crate::env::Env;
use crate::utils;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub(crate) struct BindingUsage {
    pub name: String,
}

impl BindingUsage {
    pub(crate) fn new(name: &str) -> Self {
        BindingUsage {
            name: name.to_string(),
        }
    }

    pub fn parse(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let (s, name) = utils::extract_ident(s)?;
        Ok((
            s,
            Self {
                name: name.to_string(),
            },
        ))
    }

    pub fn eval(&self, env: &Env) -> Result<Val, String> {
        env.get_binding(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            .eval(&env),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::parse("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string()
                }
            ))
        )
    }

    #[test]
    fn parse_binding_usage_with_whitespace() {
        assert_eq!(
            BindingUsage::parse("\r\na"),
            Ok((
                "",
                BindingUsage {
                    name: "a".to_string()
                }
            ))
        );

        assert_eq!(
            BindingUsage::parse("     b"),
            Ok((
                "",
                BindingUsage {
                    name: "b".to_string()
                }
            ))
        );
    }
}
