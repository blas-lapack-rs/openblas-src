# openblas-src [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides a source of [BLAS] and [LAPACK] via [OpenBLAS].

## [Architecture]

## Configuration

The following Cargo features are supported:

* `cache` to build in a shared directory instead of `target` (see below),
* `cblas` to build CBLAS (enabled by default),
* `lapacke` to build LAPACKE (enabled by default),
* `static` to link to OpenBLAS statically, and
* `system` to skip building the bundled OpenBLAS.

## Caching

The `cache` feature allows the OpenBLAS build products to be reused between
crates that have different `target` directories. This avoids rebuilding OpenBLAS
unnecessarily. However, this also prevents `cargo clean` from working properly,
since the aforementioned build products will not be removed by the command.

The OpenBLAS binary will be placed at `${XDG_DATA_HOME}/openblas_build/[hash of
build configure object]`. For example, build with LAPACK and build without
LAPACK will be placed on different directories. If you build OpenBLAS as a
shared library, you need to add the above directory to `LD_LIBRARY_PATH` (for
Linux) or `DYLD_LIBRARY_PATH` (for macOS). Since build from source is not
supported on Windows (see next section), this feature is also not supported.

## Windows and vcpkg

On Windows, `openblas-src` relies on [vcpkg] to find OpenBLAS. Before building,
you must have the correct OpenBLAS installed for your target triplet and kind of
linking. For instance, to link dynamically for the `x86_64-pc-windows-msvc`
toolchain, install `openblas` for the `x64-windows` triplet:

```sh
vcpkg install openblas --triplet x64-windows
```

To link OpenBLAS statically, install `openblas` for the `x64-windows-static-md` triplet:

```sh
vcpkg install openblas --triplet x64-windows-static-md
```

To link OpenBLAS and C Runtime (CRT) statically, install `openblas` for the `x64-windows-static` triplet:

```sh
vcpkg install openblas --triplet x64-windows-static
```

and build with `+crt-static` option

```
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-pc-windows-msvc
```

Please see the ["Static and dynamic C runtimes" in The Rust reference](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes) for detail.

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

[architecture]: https://blas-lapack-rs.github.io/architecture
[blas]: https://en.wikipedia.org/wiki/BLAS
[lapack]: https://en.wikipedia.org/wiki/LAPACK
[openblas]: http://www.openblas.net/
[openblas-cross-compile]: https://github.com/xianyi/OpenBLAS#cross-compile
[vcpkg]: https://github.com/Microsoft/vcpkg

[build-img]: https://github.com/blas-lapack-rs/openblas-src/workflows/Rust/badge.svg
[build-url]: https://github.com/blas-lapack-rs/openblas-src/actions?query=workflow%3ARust
[documentation-img]: https://docs.rs/openblas-src/badge.svg
[documentation-url]: https://docs.rs/openblas-src
[package-img]: https://img.shields.io/crates/v/openblas-src.svg
[package-url]: https://crates.io/crates/openblas-src
