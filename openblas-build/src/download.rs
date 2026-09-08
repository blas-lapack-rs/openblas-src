use anyhow::Result;
use std::path::{Path, PathBuf};
use ureq::{
    config::Config,
    tls::{TlsConfig, TlsProvider},
};

#[cfg(not(any(feature = "rustls", feature = "native-tls")))]
compile_error!("openblas-build requires the `rustls` or `native-tls` feature to be enabled");

const OPENBLAS_VERSION: &str = "0.3.34";

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
            .into_body()
            .into_reader();
        let gz_stream = flate2::read::GzDecoder::new(buf);
        let mut ar = tar::Archive::new(gz_stream);
        ar.unpack(out_dir)?;
        assert!(dest.exists());
    }
    Ok(dest)
}

fn get_agent() -> ureq::Agent {
    #[cfg(feature = "native-tls")]
    let provider = TlsProvider::NativeTls;
    #[cfg(all(feature = "rustls", not(feature = "native-tls")))]
    let provider = TlsProvider::Rustls;

    Config::builder()
        .tls_config(TlsConfig::builder().provider(provider).build())
        .build()
        .new_agent()
}
