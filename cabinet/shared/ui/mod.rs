//! The cabinet's UI kit is the shared dep-light kit, re-exported from
//! [`ev::uikit`](https://github.com/EV-invest/lib) so `crate::shared::ui::*`
//! keeps resolving. The local component copies were migrated into that crate.
pub use ev::uikit::*;
