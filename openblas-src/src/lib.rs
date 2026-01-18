// Use `CARGO_PKG_README` because of https://github.com/rust-lang/cargo/issues/11597
#![doc = include_str!(concat!("../", env!("CARGO_PKG_README")))]
#![no_std]
