//! Parses all expressions in preparsed AST to obtain untyped AST.

use super::preparser::ast as unparsed_ast;
use crate::ast::untyped as untyped_ast;
use syn::{Error, Result};

use std::convert::{TryFrom, TryInto};

impl TryFrom<unparsed_ast::Assertion> for untyped_ast::Assertion {
    type Error = Error;
    fn try_from(assertion: unparsed_ast::Assertion) -> Result<Self> {
        let untyped_assertion = match assertion {
            unparsed_ast::Assertion::Expression(expr) => {
                let untyped_expr = untyped_ast::Expression {
                    id: expr.id,
                    body: syn::parse2(expr.body)?,
                };
                untyped_ast::Assertion::Expression(untyped_expr)
            }
            unparsed_ast::Assertion::Conjunction(conjunction) => {
                let mut conjuncts = Vec::new();
                for conjunct in conjunction.conjuncts {
                    conjuncts.push(conjunct.try_into()?);
                }
                untyped_ast::Assertion::Conjunction(untyped_ast::Conjunction { conjuncts })
            }
            unparsed_ast::Assertion::Implication(implication) => {
                let untyped_implication = untyped_ast::Implication {
                    lhs: Box::new((*implication.lhs).try_into()?),
                    rhs: Box::new((*implication.rhs).try_into()?),
                };
                untyped_ast::Assertion::Implication(untyped_implication)
            }
        };
        Ok(untyped_assertion)
    }
}
