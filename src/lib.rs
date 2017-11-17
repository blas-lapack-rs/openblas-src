//! Implementation of [BLAS] and [LAPACK] via [OpenBLAS]
//!
//! The usage of the package is explained [here][usage].
//!
//! The following Cargo features are supported:
//!
//! * `cblas` to build CBLAS (enabled by default),
//! * `lapacke` to build LAPACKE (enabled by default),
//! * `static` to link to OpenBLAS statically, and
//! * `system` to skip building the bundled OpenBLAS.
//!
//! [blas]: https://en.wikipedia.org/wiki/BLAS
//! [lapack]: https://en.wikipedia.org/wiki/LAPACK
//! [openblas]: http://www.openblas.net
//! [usage]: https://blas-lapack-rs.github.io/usage

#![no_std]
