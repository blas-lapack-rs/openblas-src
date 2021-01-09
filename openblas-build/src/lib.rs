//! Helper crate for openblas-src/build.rs
//!
//! The `make` system of [OpenBLAS][OpenBLAS] has large number of inputs,
//! and detects environmental informations.
//!
//! Requirements
//! ------------
//!
//! This crate executes `make` as external command,
//! and inspects its deliverables using [GNU binutils][binutils] (`nm` and `objdump`).
//!
//! [binutils]: https://www.gnu.org/software/binutils/
//! [OpenBLAS]: https://github.com/xianyi/OpenBLAS

mod build;
mod check;
pub mod error;
pub use build::*;
pub use check::*;
