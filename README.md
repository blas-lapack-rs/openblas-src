# openblas-src [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides a source of [BLAS] and [LAPACK] via [OpenBLAS].

## [Architecture]

## Configuration

The following Cargo features are supported:

* `cache` to build in a shared directory instead of `target` (see below),
* `cblas` to build CBLAS (enabled by default),
* `lapacke` to build LAPACKE (enabled by default),
* `static` to link to OpenBLAS statically,
* `system` to skip building the bundled OpenBLAS.

Note: On Windows, OpenBLAS can not be built from source. The `system` feature is 
supposed to be used.

## Dependencies

If you want to build OpenBLAS from source, you need to have the following dependencies
installed:

* HOSTCC compiler (e.g., `gcc`, `clang`, or `icc`),
* `make`,
* CC compiler of the target architecture (e.g., `aarch64-linux-gnu-gcc` for `aarch64`),
* Fortran compiler of the target architecture(e.g., `gfortran`, `flang`, or `ifort`),
if there is no Fortran compiler detected, the flag `NOFORTRAN` should be set to `1`
and `OpenBLAS` will only compile BLAS and f2c-converted LAPACK. For more information,
please refer to the [Use f2c translations of LAPACK when no Fortran compiler is available][f2c-translations].

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

To link OpenBLAS and C Runtime (CRT) statically, install `openblas` for the 
`x64-windows-static` triplet:

```sh
vcpkg install openblas --triplet x64-windows-static
```

and build with `+crt-static` option

```sh
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-pc-windows-msvc
```

Please see the ["Static and dynamic C runtimes" in The Rust reference][crt-static] for detail.

## ENV variables

### Proxy

The `openblas-src` crate will detect and use proxy settings from your environment
variables, such as `http_proxy` and `https_proxy` to download necessary dependencies.

### Build System through OpenBLAS

According to the [OpenbLAS build system], the variables used by OpenBLAS could be
passed through environment, such as `DYNAMIC_LIST`, `NUM_THREADS`.

**HOWEVER**, for some of the variables, the `openblas-src` crate rename them to
others to avoid conflicts with the existing envs. The following is the list of
the variables that are renamed:

| OpenBLAS variable | openblas-src variable |
| ----------------- | --------------------- |
| TARGET            | OPENBLAS_TARGET       |
| CC                | OPENBLAS_CC           |
| FC                | OPENBLAS_FC           |
| HOSTCC            | OPENBLAS_HOSTCC       |
| RANLIB            | OPENBLAS_RANLIB       |

### Variables emitted by build.rs

This crate exports the following environment variables for downstream crates’ build scripts:

- `DEP_OPENBLAS_INCLUDE`: Absolute path to the OpenBLAS C headers directory (e.g., a directory that
      contains `cblas.h`, `lapacke.h` when enabled).
- `DEP_OPENBLAS_LIBRARY`: Absolute path to the produced OpenBLAS library artifact (e.g., `libopenblas.a`,
      `libopenblas.so`, `openblas.lib`, depending on platform/linking).

## Cross-compile

Apart from providing the `--target` option to `cargo build`, one also has to
specify the [cross-compilation variables of OpenBLAS][openblas-cross-compile].
They can be set as environment variables for `cargo build` using the `OPENBLAS_`
prefix as follows: `OPENBLAS_CC`, `OPENBLAS_FC`, `OPENBLAS_HOSTCC`, and
`OPENBLAS_TARGET`.

If you do not set these variables, the `openblas-build` will try to detect them.

For `OPENBLAS_TARGET`, the basic target that corresponds to the arch of `--target`
will be used.

| Rust target | OpenBLAS target |
| ----------- | --------------- |
| aarch64     | ARMV8           |
| arm         | ARMV6           |
| armv5te     | ARMV5           |
| armv6       | ARMV6           |
| armv7       | ARMV7           |
| loongarch64 | LOONGSONGENERIC |
| mips64      | MIPS64_GENERIC  |
| mips64el    | MIPS64_GENERIC  |
| riscv64     | RISCV64_GENERIC |
| csky        | CK860FV         |
| sparc       | SPARCV7         |

For `OPENBLAS_CC` and `OPENBLAS_HOSTCC`, the `cc` crate will be used to detect
the compiler. Please refer to the [cc documentation](https://docs.rs/cc/latest/cc/)
for more information.

For `OPENBLAS_FC`, `openblas-build` will try to detect the compiler through the
`OPENBLAS_CC` set above. It will replace the `gcc` with `gfortran`, `clang` with
`flang`, and `icc` with `ifort` and then test if the Fortran compiler exists.

Note: If there is no Fortran compiler detected, the build flag `NOFORTRAN` will
be set to `1` and `OpenBLAS` will only compile BLAS and f2c-converted LAPACK.
For more information, please refer to the 
[Use f2c translations of LAPACK when no Fortran compiler is available][f2c-translations].

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[architecture]: https://blas-lapack-rs.github.io/architecture
[blas]: https://en.wikipedia.org/wiki/BLAS
[lapack]: https://en.wikipedia.org/wiki/LAPACK
[OpenBLAS]: http://www.openmathlib.org/OpenBLAS/
[openblas-cross-compile]: http://www.openmathlib.org/OpenBLAS/docs/user_manual/#cross-compile
[OpenbLAS build system]: http://www.openmathlib.org/OpenBLAS/docs/build_system/
[vcpkg]: https://github.com/Microsoft/vcpkg
[f2c-translations]: https://github.com/OpenMathLib/OpenBLAS/pull/3539
[crt-static]: https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes

[build-img]: https://github.com/blas-lapack-rs/openblas-src/workflows/Rust/badge.svg
[build-url]: https://github.com/blas-lapack-rs/openblas-src/actions?query=workflow%3ARust
[documentation-img]: https://docs.rs/openblas-src/badge.svg
[documentation-url]: https://docs.rs/openblas-src
[package-img]: https://img.shields.io/crates/v/openblas-src.svg
[package-url]: https://crates.io/crates/openblas-src
