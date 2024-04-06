use crate::env::Env;
use crate::stmt::Stmt;
use crate::val::Val;

pub mod expr;
pub mod stmt;

pub mod env;
mod utils;
mod val;

#[derive(Debug)]
pub struct Parse(Stmt);

pub fn parse(s: &str) -> Result<Parse, String> {
    let (_, stmt) = Stmt::parse(s)?;
    Ok(Parse(stmt))
}

impl Parse {
    pub fn eval(&self, env: &mut Env) -> Result<Val, String> {
        self.0.eval(env)
    }
}
