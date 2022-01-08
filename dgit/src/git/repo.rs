use bytes::Bytes;
use itertools::Itertools;
use std::{
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
    rc::Rc,
};

use super::object::{ObjectID, ObjectReader};
use crate::{error::*, git::util::os_to_utf8};

trait Repository {
    fn objects() -> dyn Iterator<Item = dyn ObjectID>;
    fn read<ID>(id: ID) -> dyn ObjectReader;
}

#[derive(Debug)]
enum LocalRepositoryPath {
    Full(PathBuf),
    Bare(PathBuf),
}

impl LocalRepositoryPath {
    fn new<P>(path: P) -> Result<Self, LocalRepositoryError>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().canonicalize().context(Io)?;

        if path.join(".git").exists() {
            Ok(Self::Full(path))
        } else if path.join("/HEAD").exists() {
            Ok(LocalRepositoryPath::Bare(path))
        } else {
            Err(LocalRepositoryError::RepositoryNotFound { path })
        }
    }
}

/// A repository on disk.
pub struct LocalRepository {
    path: LocalRepositoryPath,
}

impl LocalRepository {
    pub fn new<P>(path: P) -> std::result::Result<Self, LocalRepositoryError>
    where
        P: AsRef<Path>,
    {
        // Ensure path exists
        if !path.as_ref().exists() {
            return Err(LocalRepositoryError::PathDoesNotExist {
                path: path.as_ref().to_owned(),
            });
        }

        Ok(LocalRepository {
            path: LocalRepositoryPath::new(path)?,
        })
    }

    pub fn repo_path(&self) -> PathBuf {
        match &self.path {
            LocalRepositoryPath::Bare(rv) => rv.clone(),
            LocalRepositoryPath::Full(base) => base.join(".git"),
        }
    }
}

// pub struct LocalObjectReader {

// }

impl LocalRepository {
    pub fn objects(&self) -> Box<dyn Iterator<Item = Result<Vec<u8>, LocalRepositoryError>>> {
        // Read top level objects/ directory
        match self.repo_path().join("objects").read_dir() {
            // Handle error if it arises
            Err(e) => Box::new(std::iter::once(Err(LocalRepositoryError::Io { source: e }))),

            // Otherwise now handle direcotry of directories, grouping objects by the first byte of their hash.
            Ok(prefixes) => Box::new(
                prefixes
                    .map_ok(|dir| {
                        // First byte of the hash
                        let prefix = {
                            let decoded = hex::decode(os_to_utf8(dir.file_name())?).context(Hex)?;
                            if decoded.len() == 1 {
                                decoded[0]
                            } else {
                                return Err(LocalRepositoryError::InvalidObjectDBFormat {
                                    path: dir.path(),
                                });
                            }
                        };

                        // List the prefix directory
                        let files = dir.path().read_dir().context(Io)?;

                        // Parse filenames
                        Ok(files
                            .map_ok(move |object| {
                                // Ensure object is actually a file
                                if !object.metadata().context(Io)?.is_file() {
                                    return Err(LocalRepositoryError::NotAFile {
                                        path: object.path(),
                                    });
                                }

                                // Decode the rest of the object hash
                                let mut suffix =
                                    hex::decode(os_to_utf8(object.file_name())?).context(Hex)?;

                                let mut rv = Vec::with_capacity(1 + suffix.len());
                                rv.push(prefix);
                                rv.append(&mut suffix);

                                Ok(rv)
                            })
                            .flatten())
                    })
                    .flatten()
                    .flatten_ok()
                    .flatten(),
            ),
        }
    }

    fn read(id: Vec<u8>) {
        todo!()
    }
}
