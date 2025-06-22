# CHANGELOG

[Unreleased](https://github.com/blas-lapack-rs/openblas-src/compare/openblas-src-v0.10.12...master)
-----------

[0.10.12 - 2025-06-21](https://github.com/blas-lapack-rs/openblas-src/compare/openblas-src-v0.10.11...v0.10.12)
-----------

### What's Changed

* Remove unused deps by @Dirreke in https://github.com/blas-lapack-rs/openblas-src/pull/138
* Bump ureq from 2 to 3. Bump openblas from 0.3.28 to 0.3.30 by @Dirreke in https://github.com/blas-lapack-rs/openblas-src/pull/137

[0.10.11 - 2024-12-19](https://github.com/blas-lapack-rs/openblas-src/compare/openblas-src-v0.10.10...v0.10.11)
-----------

### What's Changed

* Set MSRV to 1.71.1 and add CI to test MSRV by @Dirreke https://github.com/blas-lapack-rs/openblas-src/pull/134

[0.10.10 - 2024-12-08](https://github.com/blas-lapack-rs/openblas-src/compare/openblas-src-v0.10.9...v0.10.10)
-----------

### What's Changed

* Take proxy environment variables into account by @xoolive in https://github.com/blas-lapack-rs/openblas-src/pull/120
* rewrite CI by @gkobeaga and @Dirreke in https://github.com/blas-lapack-rs/openblas-src/pull/123
* Update OpenBLAS to version 0.3.28 by @Dirreke in https://github.com/blas-lapack-rs/openblas-src/pull/119
* Use pkg-config and fix build on doc.rs by @Fuuzetsu ,@j-baker and @HenrikJStromberg in https://github.com/blas-lapack-rs/openblas-src/pull/125
* Add cache for windows CI by @Dirreke in https://github.com/blas-lapack-rs/openblas-src/pull/128
Refactor code, Drop LAPACKE inspection, Drop FORTRAN check by @Dirreke in https://github.com/blas-lapack-rs/openblas-src/pull/127
* Detect TARGET, CC, HOSTCC, FC automically when cross-compiling by @Dirreke in https://github.com/blas-lapack-rs/openblas-src/pull/129
* Update README and CHANGELOGS by @Dirreke in  https://github.com/blas-lapack-rs/openblas-src/pull/130

[0.10.9 - 2024-02-03](https://github.com/blas-lapack-rs/openblas-src/compare/openblas-src-v0.10.8...openblas-src-v0.10.9)
--------------------

### What's Changed

* Use ubuntu 22.04 image on CI by @termoshtt in https://github.com/blas-lapack-rs/openblas-src/pull/110
* OpenBLAS v0.3.25 & Extended Target Support & Build Fixes by @gkobeaga in https://github.com/blas-lapack-rs/openblas-src/pull/108
* add rerun-if flags by @Dirreke in https://github.com/blas-lapack-rs/openblas-src/pull/105
* respect OPENBLAS_{{CC, FC, HOSTCC}} env vars on linux by @mike-kfed in https://github.com/blas-lapack-rs/openblas-src/pull/102
* Use macos-14 instance for CI by @termoshtt in https://github.com/blas-lapack-rs/openblas-src/pull/112

[0.10.8 - 2024-02-03](https://github.com/blas-lapack-rs/openblas-src/compare/openblas-src-v0.10.7...openblas-src-v0.10.8)
--------------------

### What's Changed

* Use native-tls/native-certs features of ureq crate by @lazareviczoran in https://github.com/blas-lapack-rs/openblas-src/pull/98

[0.10.7 - 2023-01-14](https://github.com/blas-lapack-rs/openblas-src/compare/openblas-src-v0.10.5...openblas-src-v0.10.7)
--------------------

0.10.6 has been yanked

### What's Changed

* homebrew directory depends on architecture. Include libomp libs by @maparent in https://github.com/blas-lapack-rs/openblas-src/pull/89
* Use `brew --prefix` command to get library path by @termoshtt in https://github.com/blas-lapack-rs/openblas-src/pull/93
* Upgrade OpenBLAS to 0.3.21 by @termoshtt in https://github.com/blas-lapack-rs/openblas-src/pull/92
* Use tar.gz image of OpenBLAS by @termoshtt in https://github.com/blas-lapack-rs/openblas-src/pull/95
* Expand tar.gz on `OUT_DIR` by @termoshtt in https://github.com/blas-lapack-rs/openblas-src/pull/96
* Download OpenBLAS source code from GitHub Release in build.rs by @termoshtt in https://github.com/blas-lapack-rs/openblas-src/pull/97

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
