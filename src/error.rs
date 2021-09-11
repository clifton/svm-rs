use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolcVmError {
    #[error("Unknown version provided")]
    UnknownVersion,
    #[error("Unsupported version {0} for platform {1}")]
    UnsupportedVersion(String, String),
    #[error("Version {0} not installed")]
    VersionNotInstalled(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    UrlError(#[from] url::ParseError),
}