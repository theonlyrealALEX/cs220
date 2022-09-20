//! Parser.

//use self::inner::SyntaxParser;

use self::inner::SyntaxParser;

use super::syntax::*;
use anyhow::Result;

#[allow(missing_docs)]
#[allow(missing_debug_implementations)]
mod inner {
    use pest_derive::*;

    #[derive(Parser)]
    #[grammar = "assignments/assignment04/syntax.pest"]
    pub(crate) struct SyntaxParser;
}

/// Parses command.
///
/// ## Operator Associativty
///
/// For associativity of each operator, please follow [here](https://docs.rs/pest/latest/pest/prec_climber/struct.PrecClimber.html#examples).
///
/// e.g. `1+2+3` should be parsed into `(1+2)+3`, not `1+(2+3)` because the associativity of plus("add" in our hw) operator is `Left`.
pub fn parse_command(line: &str) -> Result<Command> {
    let succesfull_parse = SyntaxParser::parse(Rule::command, line);

    todo!("fill here")
}
