//! Domain core: entities, value objects, ports, and errors.
//!
//! This layer is pure — it has no knowledge of HTTP, SQL, or any concrete
//! framework. Outer layers depend inward on it; it depends on nothing of theirs.

pub mod error;
pub mod model;
pub mod port;
