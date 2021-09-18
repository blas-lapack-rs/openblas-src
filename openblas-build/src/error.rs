use std::{io, path::*, process::Command};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Subprocess returns with non-zero status: {status}")]
    NonZeroExitStatus { status: i32 },

    #[error("Subprocess cannot start: {error:?}")]
    SubprocessCannotStart { error: io::Error },

    #[error("Fortran compiler not found. It is necessary to build LAPACK.")]
    FortranCompilerNotFound,

    #[error("Cannot canonicalize path in Linker flag: {}", path.display())]
    CannotCanonicalizePath { path: PathBuf },

    #[error("Makefile.conf does not exist in {}", out_dir.display())]
    MakeConfNotExist { out_dir: PathBuf },

    #[error("Library file does not exist: {}", path.display())]
    LibraryNotExist { path: PathBuf },

    #[error("Target {} is unsupported", target)]
    UnsupportedTarget { target: String },

    #[error("Other IO errors: {0:?}")]
    IOError(#[from] io::Error),
}

pub(crate) trait CheckCall {
    fn check_call(&mut self) -> Result<(), Error>;
}

impl CheckCall for Command {
    fn check_call(&mut self) -> Result<(), Error> {
        match self.status() {
            Ok(status) => {
                if !status.success() {
                    Err(Error::NonZeroExitStatus {
                        status: status.code().unwrap_or(-1),
                    })
                } else {
                    Ok(())
                }
            }
            Err(error) => Err(Error::SubprocessCannotStart { error }),
        }
    }
}
