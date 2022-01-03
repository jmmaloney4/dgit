use std::path::PathBuf;

use snafu::Snafu;
pub(crate) use snafu::{OptionExt, ResultExt};

// pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {}

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum LocalRepositoryError {
    #[snafu(display("Io error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Couldn't convert bytes to utf8: {}", source))]
    Utf8 { source: std::string::FromUtf8Error },

    #[snafu(display("Couldn't convert OsString to utf8: {:?}", os_str))]
    String { os_str: std::ffi::OsString },

    #[snafu(display("Couldn't decode hex string: {}", source))]
    Hex { source: hex::FromHexError },

    #[snafu(display("Path does not exist: {:#?}", path))]
    PathDoesNotExist { path: PathBuf },

    #[snafu(display("Couldn't find a repository at the path: {:#?}", path))]
    RepositoryNotFound { path: PathBuf },

    #[snafu(display("Object DB entry is not a file: {:?}", path))]
    NotAFile { path: PathBuf },

    #[snafu(display("Object DB entry is not valid: {:?}", path))]
    InvalidObjectDBFormat { path: PathBuf },
}
