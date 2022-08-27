These versions are based on [semantic versioning][semver], and corresponds to the version in [openblas-src/Cargo.toml](openblas-src/Cargo.toml).

[semver]: https://semver.org/

Unreleased
-----------

0.10.5 - 2022-08-27
--------------------

From this release, `openblas-build` crate will have same version as `openblas-src` crate.

### Fixed
- Add support for using a custom target under linux https://github.com/blas-lapack-rs/openblas-src/pull/78

### Changed
- OpenBLAS 0.3.20 https://github.com/blas-lapack-rs/openblas-src/pull/85
  - OpenBLAS 0.3.17 https://github.com/blas-lapack-rs/openblas-src/pull/76
- Use dynamic CRT link for vcpkg https://github.com/blas-lapack-rs/openblas-src/pull/69 https://github.com/blas-lapack-rs/openblas-src/pull/71

### Internal
- Run cargo-clippy and rustfmt on CI https://github.com/blas-lapack-rs/openblas-src/pull/86

0.10.4 - 2021-04-03
--------------------

### Fixed
- Change link search order https://github.com/blas-lapack-rs/openblas-src/pull/61

0.10.3 - 2021-04-02
-------------------

### Fixed
- Update "cache" feature description https://github.com/blas-lapack-rs/openblas-src/pull/63

### Changed
- Upgrade OpenBLAS to 0.3.14  https://github.com/blas-lapack-rs/openblas-src/pull/65

0.10.2 - 2021-01-30
--------------------

0.10.0 and 0.10.1 has been yanked and changes from 0.9.0 is summarized here.

### Fixed
- Detect OpenBLAS does not build some parts of LAPACK while "lapack" feature is enabled https://github.com/blas-lapack-rs/openblas-src/issues/49
  - openblas-build crate has been introduced to resolve this issue

### Added
- openblas-build crate is introduced to sneak OpenBLAS build system configure
  - Link test for LAPACK routines written in Fortran https://github.com/blas-lapack-rs/openblas-src/pull/43
  - Switch to openblas-build on Linux https://github.com/blas-lapack-rs/openblas-src/pull/52
    - Not on macOS due to https://github.com/blas-lapack-rs/openblas-src/issues/54
  - Create openblas-build crate https://github.com/blas-lapack-rs/openblas-src/pull/47
    - cargo-workspace setup https://github.com/blas-lapack-rs/openblas-src/pull/45
- "system" feature support for windows-msvc target through vcpkg https://github.com/blas-lapack-rs/openblas-src/pull/35

### Changed
- Use Rust 2018 edition https://github.com/blas-lapack-rs/openblas-src/pull/46
- Switch to GitHub Actions from AppVeyor + Travis CI https://github.com/blas-lapack-rs/openblas-src/pull/40

0.9.0 - 2020-03-08
--------------------

### Changed
- Build products are placed on OUT_DIR to work `cargo clean` properly https://github.com/blas-lapack-rs/openblas-src/pull/31
  - Previous behavior (placed on .cargo/) is reproduced with "cache" feature
