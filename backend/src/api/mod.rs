//! API: the driving HTTP adapter (axum). Translates requests into use-case
//! calls and domain results into HTTP responses. DTOs keep the wire format
//! decoupled from domain models, and `openapi` derives the contract from them.

pub mod dto;
pub mod error;
pub mod handler;
pub mod openapi;
pub mod router;
pub mod state;
