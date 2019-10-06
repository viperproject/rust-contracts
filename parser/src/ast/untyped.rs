//! Untyped AST.

use quote::ToTokens;
use std::fmt;
use syn;

use super::common;

pub type ExprType = syn::Expr;

pub type Assertion = common::Assertion<ExprType>;
pub type Expression = common::Expression<ExprType>;
pub type Conjunction = common::Conjunction<ExprType>;
pub type Implication = common::Implication<ExprType>;

impl fmt::Display for Assertion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A(")?;
        match self {
            Assertion::Expression(expr) => {
                write!(f, "{}", expr)?;
            }
            Assertion::Conjunction(conjunction) => {
                write!(f, "{}", conjunction)?;
            }
            Assertion::Implication(implication) => {
                write!(f, "{}", implication)?;
            }
        }
        write!(f, ")")
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E({})", self.body.to_token_stream())
    }
}

impl fmt::Display for Conjunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C(")?;
        let mut iter = self.conjuncts.iter();
        write!(f, "{}", iter.next().expect("Empty conjunction?"))?;
        for conjunct in iter {
            write!(f, " && {}", conjunct)?;
        }
        write!(f, ")")
    }
}

impl fmt::Display for Implication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I({} ==> {})", self.lhs, self.rhs)
    }
}
