These versions are based on [semantic versioning][semver], and corresponds to the version in `Cargo.toml`.

[semver]: https://semver.org/

Unreleased
-----------

0.10.4 - 2021-04-03
--------------------

### Fixed
- Change link search order #61

0.10.3 - 2021-04-02
-------------------

### Fixed
- Update "cache" feature description #63

### Changed
- Upgrade OpenBLAS to 0.3.14  #65

0.10.2 - 2021-01-30
--------------------

0.10.0 and 0.10.1 has been yanked and changes from 0.9.0 is summarized here.

### Fixed
- Detect OpenBLAS does not build some parts of LAPACK while "lapack" feature is enabled #49
  - openblas-build crate has been introduced to resolve this issue

### Added
- openblas-build crate is introduced to sneak OpenBLAS build system configure
  - Link test for LAPACK routines written in Fortran #43
  - Switch to openblas-build on Linux #52
    - Not on macOS due to #54
  - Create openblas-build crate #47
    - cargo-workspace setup #45

### Changed
- Use Rust 2018 edition #46
- Switch to GitHub Actions from AppVeyor + Travis CI #40
