use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
/// A unique identifier of the Rust expression used in the specification.
pub struct ExprId(Uuid);

impl ExprId {
    /// Constructor.
    pub fn new() -> Self {
        Self { 0: Uuid::new_v4() }
    }
}

impl ToString for ExprId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
