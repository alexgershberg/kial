use crate::env::Env;
use crate::expr::block::Block;
use crate::utils;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionDef {
    name: String,
    args: Vec<String>,
    body: Block,
}

impl FunctionDef {
    pub(crate) fn new(name: &str, args: Vec<String>, body: Block) -> Self {
        Self {
            name: name.to_string(),
            args,
            body,
        }
    }

    pub(crate) fn parse(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("func", s)?;
        let (s, _) = utils::extract_whitespace1(s)?;

        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("(", s)?;
        let (s, _) = utils::extract_whitespace(s);

        // Parse comma separated args
        let mut s = s;
        let mut args = vec![];
        while let Ok((new_s, ident)) = utils::extract_ident(s) {
            args.push(ident.to_string());
            s = new_s;

            (s, _) = utils::extract_whitespace(s);

            if let Ok(new_s) = utils::tag(",", s) {
                s = new_s;
            } else {
                break;
            }
        }

        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag(")", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, body) = Block::parse(s)?;
        let (s, _) = utils::extract_whitespace(s);

        Ok((
            s,
            Self {
                name: name.to_string(),
                args: args,
                body,
            },
        ))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Val, String> {
        let val = self.body.eval(env)?;
        env.store_binding(self.name.to_string(), val);

        Ok(Val::Unit)
    }
}

#[cfg(test)]
mod tests {
    use crate::env::Env;
    use crate::expr::binding_usage::BindingUsage;
    use crate::expr::block::Block;
    use crate::expr::Expr;
    use crate::expr::Number;
    use crate::stmt::binding_def::BindingDef;
    use crate::stmt::{FunctionDef, Stmt};
    use crate::val::Val;

    #[test]
    fn parse_function_empty_body_no_args() {
        assert_eq!(
            FunctionDef::parse(
                "\
        func hello_world() {
        }"
            ),
            Ok((
                "",
                FunctionDef::new("hello_world", vec![], Block::new(vec![]))
            ))
        )
    }

    #[test]
    fn parse_function_no_args_compact() {
        assert_eq!(
            FunctionDef::parse("func hello_world(){let a=200; a}"),
            Ok((
                "",
                FunctionDef::new(
                    "hello_world",
                    vec![],
                    Block::new(vec![
                        Stmt::BindingDef(BindingDef::new("a", Expr::Number(Number(200)))),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage::new("a")))
                    ])
                )
            ))
        )
    }

    #[test]
    fn parse_function_no_args_malformed() {
        assert_eq!(
            FunctionDef::parse(
                "\
        func           
          
          hello_world(
        )       {
            let a = 200;
            a
        }         "
            ),
            Ok((
                "",
                FunctionDef::new(
                    "hello_world",
                    vec![],
                    Block::new(vec![
                        Stmt::BindingDef(BindingDef::new("a", Expr::Number(Number(200)))),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage::new("a")))
                    ])
                )
            ))
        )
    }

    #[test]
    fn parse_function_no_args() {
        assert_eq!(
            FunctionDef::parse(
                "\
        func hello_world() {
            let a = 200;
            a
        }"
            ),
            Ok((
                "",
                FunctionDef::new(
                    "hello_world",
                    vec![],
                    Block::new(vec![
                        Stmt::BindingDef(BindingDef::new("a", Expr::Number(Number(200)))),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage::new("a")))
                    ])
                )
            ))
        )
    }

    #[test]
    fn parse_function_1_arg() {
        assert_eq!(
            FunctionDef::parse(
                "\
        func hello_world(one) {
            let a = 200;
            a
        }"
            ),
            Ok((
                "",
                FunctionDef::new(
                    "hello_world",
                    vec!["one".to_string()],
                    Block::new(vec![
                        Stmt::BindingDef(BindingDef::new("a", Expr::Number(Number(200)))),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage::new("a")))
                    ])
                )
            ))
        )
    }

    #[test]
    fn parse_function_1_arg_trailing_args_comma() {
        assert_eq!(
            FunctionDef::parse(
                "\
        func hello_world(one, ) {
            let a = 200;
            a
        }"
            ),
            Ok((
                "",
                FunctionDef::new(
                    "hello_world",
                    vec!["one".to_string()],
                    Block::new(vec![
                        Stmt::BindingDef(BindingDef::new("a", Expr::Number(Number(200)))),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage::new("a")))
                    ])
                )
            ))
        )
    }

    #[test]
    fn parse_function_2_args() {
        assert_eq!(
            FunctionDef::parse(
                "\
        func hello_world(one, two) {
            let a = 200;
            a
        }"
            ),
            Ok((
                "",
                FunctionDef::new(
                    "hello_world",
                    vec!["one".to_string(), "two".to_string()],
                    Block::new(vec![
                        Stmt::BindingDef(BindingDef::new("a", Expr::Number(Number(200)))),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage::new("a")))
                    ])
                )
            ))
        )
    }

    #[test]
    fn parse_function_5_args() {
        assert_eq!(
            FunctionDef::parse(
                "\
        func hello_world(one, two, three, four, five) {
            let a = 200;
            a
        }"
            ),
            Ok((
                "",
                FunctionDef::new(
                    "hello_world",
                    vec![
                        "one".to_string(),
                        "two".to_string(),
                        "three".to_string(),
                        "four".to_string(),
                        "five".to_string()
                    ],
                    Block::new(vec![
                        Stmt::BindingDef(BindingDef::new("a", Expr::Number(Number(200)))),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage::new("a")))
                    ])
                )
            ))
        )
    }

    #[test]
    fn parse_function_5_args_malformed() {
        assert_eq!(
            FunctionDef::parse(
                "\
        func 
        hello_world
        
        (
        
        
        one
        ,\r\n
         two
         
         , \t
         three
         
         , \r\n four,five
         
         
         
         ) {
            let a = 200;



            a
        }"
            ),
            Ok((
                "",
                FunctionDef::new(
                    "hello_world",
                    vec![
                        "one".to_string(),
                        "two".to_string(),
                        "three".to_string(),
                        "four".to_string(),
                        "five".to_string()
                    ],
                    Block::new(vec![
                        Stmt::BindingDef(BindingDef::new("a", Expr::Number(Number(200)))),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage::new("a")))
                    ])
                )
            ))
        )
    }

    #[test]
    fn parse_function_5_args_compact() {
        assert_eq!(
            FunctionDef::parse("func hello_world(one,two,three,four,five){let a=200;a}"),
            Ok((
                "",
                FunctionDef::new(
                    "hello_world",
                    vec![
                        "one".to_string(),
                        "two".to_string(),
                        "three".to_string(),
                        "four".to_string(),
                        "five".to_string()
                    ],
                    Block::new(vec![
                        Stmt::BindingDef(BindingDef::new("a", Expr::Number(Number(200)))),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage::new("a")))
                    ])
                )
            ))
        )
    }

    #[test]
    fn eval_function_no_args() {
        let mut env = Env::default();
        assert_eq!(
            FunctionDef::new(
                "hello_world",
                vec![],
                Block::new(vec![
                    Stmt::BindingDef(BindingDef::new("a", Expr::Number(Number(200)))),
                    Stmt::Expr(Expr::BindingUsage(BindingUsage::new("a"))),
                ]),
            )
            .eval(&mut env),
            Ok(Val::Unit)
        );

        assert_eq!(env.get_binding("hello_world"), Ok(Val::Number(200)));
    }
}
