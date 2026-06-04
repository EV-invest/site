//! Infrastructure: driven adapters that implement the domain ports against
//! concrete technologies (Postgres via sqlx, etc.).

pub mod db;
pub mod persistence;
