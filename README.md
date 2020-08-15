# openblas-src [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build1-img]][build1-url] [![Build][build2-img]][build2-url]

The package provides a source of [BLAS] and [LAPACK] via [OpenBLAS].

The usage of the package is explained [here][usage].

The following Cargo features are supported:

* `cache` to build in `.cargo` instead of `target` (see below),
* `cblas` to build CBLAS (enabled by default),
* `lapacke` to build LAPACKE (enabled by default),
* `static` to link to OpenBLAS statically, and
* `system` to skip building the bundled OpenBLAS.

## Caching

The `cache` feature allows the OpenBLAS build products to be reused between
crates that have different `target` directories. This avoids rebuilding OpenBLAS
unnecessarily. However, this also prevents `cargo clean` from working properly,
since the aforementioned build products will not be removed by the command.

## Windows and vcpkg

On Windows, `openblas-src` relies on [vcpkg] to find OpenBLAS. Before building,
you must have the correct OpenBLAS installed for your target triplet and kind of
linking. For instance, to link dynamically for the `x86_64-pc-windows-msvc`
toolchain, install `openblas` for the `x64-windows` triplet:

```sh
vcpkg install openblas --triplet x64-windows
```

To link statically, install `openblas` for the `x64-windows-static` triplet:

```sh
vcpkg install openblas --triplet x64-windows-static
```

## Cross Compilation

Apart from providing the `--target` option to `cargo build`, one also has to
specify the [cross-compilation variables of OpenBLAS][openblas-cross-compile].
They can be set as environment variables for `cargo build` using the `OPENBLAS_`
prefix as follows: `OPENBLAS_CC`, `OPENBLAS_FC`, `OPENBLAS_HOSTCC`, and
`OPENBLAS_TARGET`.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[blas]: https://en.wikipedia.org/wiki/BLAS
[lapack]: https://en.wikipedia.org/wiki/LAPACK
[openblas]: http://www.openblas.net/
[openblas-cross-compile]: https://github.com/xianyi/OpenBLAS#cross-compile
[usage]: https://blas-lapack-rs.github.io/usage
[vcpkg]: https://github.com/Microsoft/vcpkg

[build1-img]: https://travis-ci.org/blas-lapack-rs/openblas-src.svg?branch=master
[build1-url]: https://travis-ci.org/blas-lapack-rs/openblas-src
[build2-img]: https://ci.appveyor.com/api/projects/status/l438i5ov5wsw4qw0?svg=true
[build2-url]: https://ci.appveyor.com/project/IvanUkhov/openblas-src
[documentation-img]: https://docs.rs/openblas-src/badge.svg
[documentation-url]: https://docs.rs/openblas-src
[package-img]: https://img.shields.io/crates/v/openblas-src.svg
[package-url]: https://crates.io/crates/openblas-src
