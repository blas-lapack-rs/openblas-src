# openblas-provider

Bindings to OpenBLAS-specific functions, and linking to OpenBLAS. By default,
this crate will build and use a bundled OpenBLAS. Building this will require a
Fortran and C compiler available. This provides BLAS and LAPACK.

Two Cargo features are supported:

- `static-openblas`: link to OpenBLAS statically.
- `system-openblas`: don't use the bundled OpenBLAS.

# Where are all the FFI definitions?

This crate only provides a BLAS implementation. Bindings are available at https://github.com/stainless-steel/libblas-sys,
and a wrapper is available at https://github.com/stainless-steel/blas.
