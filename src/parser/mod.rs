//! Parser module for MiniC language.

pub mod expressions;
pub mod identifiers;
pub mod literals;
pub mod statements;

pub use expressions::expression;
pub use identifiers::identifier;
pub use literals::{literal, Literal};
pub use statements::{assignment, statement};
