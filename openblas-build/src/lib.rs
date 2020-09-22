//! Helper crate for openblas-src/build.rs
//!
//! The `make` system of [OpenBLAS][OpenBLAS] has large number of inputs,
//! and detects environmental informations.
//!
//! Requirements
//! ------------
//!
//! This crate executes `make` as external command,
//! and inspects its deliverables using [GNU binutils][binutils] (`nm` and `objdump`).
//!
//! [binutils]: https://www.gnu.org/software/binutils/
//! [OpenBLAS]: https://github.com/xianyi/OpenBLAS

mod build;
mod check;
pub use build::*;
pub use check::*;

use anyhow::{bail, Result};
use std::process::Command;

pub(crate) trait CheckCall {
    fn check_call(&mut self) -> Result<()>;
}

impl CheckCall for Command {
    fn check_call(&mut self) -> Result<()> {
        match self.status() {
            Ok(status) => {
                if !status.success() {
                    bail!(
                        "Subprocess returns with non-zero status: `{:?}` ({})",
                        self,
                        status
                    );
                }
                Ok(())
            }
            Err(error) => {
                bail!("Subprocess execution failed: `{:?}` ({})", self, error);
            }
        }
    }
}
