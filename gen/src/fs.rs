#![allow(dead_code)]

use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::io;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) struct Error {
    source: io::Error,
    message: String,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}

macro_rules! err {
    ($io_error:expr, $fmt:expr $(, $path:expr)* $(,)?) => {
        Err(Error {
            source: $io_error,
            message: format!($fmt $(, $path.display())*),
        })
    }
}

pub(crate) fn canonicalize(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    match std::fs::canonicalize(path) {
        Ok(string) => Ok(string),
        Err(e) => err!(e, "Unable to canonicalize path: `{}`", path),
    }
}

pub(crate) fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<u64> {
    let from = from.as_ref();
    let to = to.as_ref();
    match std::fs::copy(from, to) {
        Ok(n) => Ok(n),
        Err(e) => err!(e, "Failed to copy `{}` -> `{}`", from, to),
    }
}

pub(crate) fn create_dir_all(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    match std::fs::create_dir_all(path) {
        Ok(()) => Ok(()),
        Err(e) => err!(e, "Failed to create directory `{}`", path),
    }
}

pub(crate) fn current_dir() -> Result<PathBuf> {
    match std::env::current_dir() {
        Ok(dir) => Ok(dir),
        Err(e) => err!(e, "Failed to determine current directory"),
    }
}

pub(crate) fn read(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let path = path.as_ref();
    match std::fs::read(path) {
        Ok(string) => Ok(string),
        Err(e) => err!(e, "Failed to read file `{}`", path),
    }
}

pub(crate) fn remove_file(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) => err!(e, "Failed to remove file `{}`", path),
    }
}

fn symlink<'a>(
    src: &'a Path,
    dst: &'a Path,
    fun: fn(&'a Path, &'a Path) -> io::Result<()>,
) -> Result<()> {
    match fun(src, dst) {
        Ok(()) => Ok(()),
        Err(e) => err!(
            e,
            "Failed to create symlink `{}` pointing to `{}`",
            dst,
            src,
        ),
    }
}

#[cfg(unix)]
#[allow(unused_imports)]
pub(crate) use self::symlink_file as symlink_dir;

#[cfg(unix)]
pub(crate) fn symlink_file(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    symlink(src.as_ref(), dst.as_ref(), std::os::unix::fs::symlink)
}

#[cfg(windows)]
pub(crate) fn symlink_file(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    symlink(
        src.as_ref(),
        dst.as_ref(),
        std::os::windows::fs::symlink_file,
    )
}

#[cfg(windows)]
pub(crate) fn symlink_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    symlink(
        src.as_ref(),
        dst.as_ref(),
        std::os::windows::fs::symlink_dir,
    )
}

pub(crate) fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Result<()> {
    let path = path.as_ref();
    match std::fs::write(path, contents) {
        Ok(()) => Ok(()),
        Err(e) => err!(e, "Failed to write file `{}`", path),
    }
}
