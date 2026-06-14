#![allow(unused_features)]
#![feature(default_field_values)]
pub mod application;
pub mod entities;
pub mod features;
pub mod shared;
pub mod views;

pub use application::router::{App, Route};
