use crate::error::LocalRepositoryError;

pub(crate) fn os_to_utf8(s: std::ffi::OsString) -> Result<String, LocalRepositoryError> {
    match s.to_str() {
        None => Err(LocalRepositoryError::String { os_str: s }),
        Some(s) => Ok(s.to_owned()),
    }
}
