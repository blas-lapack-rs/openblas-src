use anyhow::Result;
use std::path::{Path, PathBuf};

const OPENBLAS_VERSION: &str = "0.3.28";

pub fn openblas_source_url() -> String {
    format!(
        "https://github.com/OpenMathLib/OpenBLAS/releases/download/v{}/OpenBLAS-{}.tar.gz",
        OPENBLAS_VERSION, OPENBLAS_VERSION
    )
}

pub fn download(out_dir: &Path) -> Result<PathBuf> {
    let dest = out_dir.join(format!("OpenBLAS-{}", OPENBLAS_VERSION));
    if !dest.exists() {
        let buf = get_agent()
            .get(&openblas_source_url())
            .call()?
            .into_reader();
        let gz_stream = flate2::read::GzDecoder::new(buf);
        let mut ar = tar::Archive::new(gz_stream);
        ar.unpack(out_dir)?;
        assert!(dest.exists());
    }
    Ok(dest)
}

fn get_agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .tls_connector(std::sync::Arc::new(
            native_tls::TlsConnector::new().expect("failed to create TLS connector"),
        ))
        .build()
}
