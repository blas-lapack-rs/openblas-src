//! Helper crate for openblas-src/build.rs
//!
//! The `make` system of [OpenBLAS][OpenBLAS] has large number of inputs,
//! and detects environmental informations.
//!
//! [OpenBLAS]: https://github.com/xianyi/OpenBLAS

mod build;
mod check;
pub use build::*;
pub use check::*;
