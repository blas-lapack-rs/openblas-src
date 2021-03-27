//! Source of [BLAS] and [LAPACK] via [OpenBLAS].
//!
//! ## [Architecture]
//!
//! ## Configuration
//!
//! The following Cargo features are supported:
//!
//! * `cache` to build in shared directory e.g. `$XDG_DATA_HOME/openblas_build/`
//!   instead of `target` (see README),
//! * `cblas` to build CBLAS (enabled by default),
//! * `lapacke` to build LAPACKE (enabled by default),
//! * `static` to link to OpenBLAS statically, and
//! * `system` to skip building the bundled OpenBLAS.
//!
//! [architecture]: https://blas-lapack-rs.github.io/architecture
//! [blas]: https://en.wikipedia.org/wiki/BLAS
//! [lapack]: https://en.wikipedia.org/wiki/LAPACK
//! [openblas]: http://www.openblas.net

#![no_std]
