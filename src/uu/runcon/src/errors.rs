use std::ffi::OsString;
use std::fmt::Write;
use std::io;
use std::str::Utf8Error;

use uucore::display::Quotable;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("No command is specified")]
    MissingCommand,

    #[error("SELinux is not enabled")]
    SELinuxNotEnabled,

    #[error(transparent)]
    NotUTF8(#[from] Utf8Error),

    #[error(transparent)]
    CommandLine(#[from] clap::Error),

    #[error("{operation} failed")]
    SELinux {
        operation: &'static str,
        source: selinux::errors::Error,
    },

    #[error("{operation} failed")]
    Io {
        operation: &'static str,
        source: io::Error,
    },

    #[error("{operation} failed on {}", .operand1.quote())]
    Io1 {
        operation: &'static str,
        operand1: OsString,
        source: io::Error,
    },
}

impl Error {
    pub(crate) fn from_io(operation: &'static str, source: io::Error) -> Self {
        Self::Io { operation, source }
    }

    pub(crate) fn from_io1(
        operation: &'static str,
        operand1: impl Into<OsString>,
        source: io::Error,
    ) -> Self {
        Self::Io1 {
            operation,
            operand1: operand1.into(),
            source,
        }
    }

    pub(crate) fn from_selinux(operation: &'static str, source: selinux::errors::Error) -> Self {
        Self::SELinux { operation, source }
    }
}

pub(crate) fn report_full_error(mut err: &dyn std::error::Error) -> String {
    let mut desc = String::with_capacity(256);
    write!(&mut desc, "{}", err).unwrap();
    while let Some(source) = err.source() {
        err = source;
        write!(&mut desc, ": {}", err).unwrap();
    }
    desc.push('.');
    desc
}
