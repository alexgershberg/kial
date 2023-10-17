extern crate core;

use crate::env::Env;
use crate::stmt::Stmt;

pub mod binding_def;
pub mod expr;
pub mod stmt;

pub mod env;
mod utils;
mod val;

pub fn parse(s: &str, env: &mut Env) -> Result<val::Val, String> {
    let (_, stmt) = Stmt::new(s)?;
    stmt.eval(env)
}
