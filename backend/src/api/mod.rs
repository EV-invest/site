//! API: the driving HTTP adapter (axum). Translates requests into use-case
//! calls and domain results into HTTP responses. DTOs keep the wire format
//! decoupled from domain models.

pub mod dto;
pub mod error;
pub mod handler;
pub mod router;
pub mod state;
