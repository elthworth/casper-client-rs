use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use rand::{self, distributions::Alphanumeric, Rng};

use crate::Error;

/// An output abstraction for associating a [`Write`] object with some metadata.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OutputKind<'a> {
    /// An output file.
    File {
        /// The path of the output file.
        path: &'a Path,
        /// The path to a temporary file in the same directory as the output file, which is used to
        /// make the write operation transactional and ensures that the file at `path` is not
        /// damaged if it pre-exists.
        tmp_path: PathBuf,
        /// If `overwrite_if_exists` is `true`, then the file at `path` will be overwritten.
        overwrite_if_exists: bool,
    },
    /// Stdout.
    Stdout,
}

impl<'a> OutputKind<'a> {
    /// Returns a new `OutputKind::File`.
    pub fn file(path: &'a Path, overwrite_if_exists: bool) -> Self {
        let collision_resistant_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect::<String>();
        let extension = format!(".{}.tmp", &collision_resistant_string);
        let tmp_path = path.with_extension(extension);
        OutputKind::File {
            path,
            tmp_path,
            overwrite_if_exists,
        }
    }

    /// Returns a `Result` containing a `Write` trait object.
    pub(super) fn get(&self) -> Result<Box<dyn Write>, Error> {
        match self {
            OutputKind::File {
                path,
                tmp_path,
                overwrite_if_exists,
            } => {
                if path.exists() && !*overwrite_if_exists {
                    return Err(Error::FileAlreadyExists(PathBuf::from(path)));
                }
                let file = File::create(&tmp_path).map_err(|error| Error::IoError {
                    context: format!("failed to create {}", tmp_path.display()),
                    error,
                })?;

                let write: Box<dyn Write> = Box::new(file);
                Ok(write)
            }
            OutputKind::Stdout if cfg!(test) => Ok(Box::new(io::sink())),
            OutputKind::Stdout => Ok(Box::new(io::stdout())),
        }
    }

    /// When `self` is `OutputKind::File`, causes the temp file to be renamed (moved) to its `path`.
    /// When `self` is `OutputKind::Stdout` this is a no-op.
    pub(super) fn commit(self) -> Result<(), Error> {
        match self {
            OutputKind::File { path, tmp_path, .. } => {
                fs::rename(&tmp_path, path).map_err(|error| Error::IoError {
                    context: format!(
                        "could not move tmp file {} to destination {}",
                        tmp_path.display(),
                        path.display()
                    ),
                    error,
                })
            }
            OutputKind::Stdout => Ok(()),
        }
    }
}
