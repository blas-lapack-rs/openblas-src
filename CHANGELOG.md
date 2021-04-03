These versions are based on [semantic versioning][semver], and corresponds to the version in `Cargo.toml`.

[semver]: https://semver.org/

Unreleased
-----------

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
