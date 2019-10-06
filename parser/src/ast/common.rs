//! AST nodes paremetrized with the Rust expression.

use super::ids::ExprId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Assertion<ExprType> {
    Expression(Expression<ExprType>),
    Conjunction(Conjunction<ExprType>),
    //ForAll(ForAll),
    Implication(Implication<ExprType>),
    //Pledge(Pledge),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expression<ExprType> {
    pub id: ExprId,
    pub body: ExprType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conjunction<ExprType> {
    pub conjuncts: Vec<Assertion<ExprType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Implication<ExprType> {
    pub lhs: Box<Assertion<ExprType>>,
    pub rhs: Box<Assertion<ExprType>>,
}
