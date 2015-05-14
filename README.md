# OpenBLAS Provider

The package provides [BLAS][1] and [LAPACK][2] using the [OpenBLAS][3]
implementation. By default, the package will build and use a bundled OpenBLAS,
which requires a Fortran compiler.

The following two Cargo features are supported:

- `static-openblas` to link to OpenBLAS statically and
- `system-openblas` to skip building the bundled OpenBLAS.

## Where are all the FFI definitions?

This package provides only an implementation of BLAS and LAPACK. Bindings are
available in [libblas-sys][4] and [liblapack-sys][5], and wrappers are available
in [blas][6] and [lapack][7].

[1]: https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms
[2]: https://en.wikipedia.org/wiki/LAPACK
[3]: http://www.openblas.net

[4]: https://github.com/stainless-steel/libblas-sys
[5]: https://github.com/stainless-steel/liblapack-sys
[6]: https://github.com/stainless-steel/blas
[7]: https://github.com/stainless-steel/lapack
