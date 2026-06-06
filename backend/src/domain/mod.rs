//! Domain core: entities, value objects, ports, and errors.
//!
//! Models and errors live in the `domain` crate (workspace) so the CRM
//! frontend can share the same types without codegen. Re-exported here so
//! all existing `use crate::domain::...` paths in this crate stay unchanged.

pub use ::domain::error;
pub use ::domain::model;
pub mod port;
