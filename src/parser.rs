use anyhow::Result;
use lalrpop_util::lalrpop_mod;
pub mod syn;

lalrpop_mod!(syntax, "/parser/syntax.rs");

pub fn parse(cmd: &str) -> Result<syn::Command> {
    match syntax::CommandParser::new().parse(cmd) {
        Ok(v) => return Ok(v),
        Err(e) => return Err(anyhow::anyhow!("Parse error: {}", e)),
    }
}