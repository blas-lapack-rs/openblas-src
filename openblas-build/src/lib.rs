//! openblas-build
//! ---------------
//!
//! Helper crate for openblas-src/build.rs

mod build;
mod check;
pub use build::*;
pub use check::*;
