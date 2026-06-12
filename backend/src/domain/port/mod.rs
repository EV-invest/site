//! Ports: the abstract interfaces (traits) through which the core talks to the
//! outside world. Driven adapters in `crate::infrastructure` implement them.

pub mod blog_repository;
pub mod ledger;
