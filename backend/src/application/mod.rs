//! Application layer: use cases that orchestrate domain rules over the ports.
//! It knows the domain but remains ignorant of concrete adapters.

pub mod blog_service;
pub mod ledger_service;
