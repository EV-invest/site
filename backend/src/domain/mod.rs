//! Backend-local ports through which the application core reaches
//! infrastructure adapters. Domain types (models, errors) live in the
//! `domain` workspace crate and are imported directly from there.

pub mod port;
