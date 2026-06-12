//! Infrastructure: driven adapters that implement the domain ports against
//! concrete technologies (Postgres via sqlx, TigerBeetle, etc.).

pub mod db;
pub mod persistence;
pub mod tigerbeetle;
