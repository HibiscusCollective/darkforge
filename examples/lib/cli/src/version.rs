/*
 * Copyright (C) 2025 Pierre Fouilloux, Hibiscus Collective
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along with this program.
 * If not, see https://www.gnu.org/licenses/.
 */

//! # Version Module
//!
//! This module provides functionality for displaying version information.
//! It contains a simple command implementation that outputs the version string
//! to a provided writer.

use std::io::Write;

use anyhow::anyhow;

/// A command that displays version information.
///
/// This struct holds a version string and provides functionality to output
/// it to any writer that implements the `Write` trait.
pub struct VersionCmd<'a>(&'a str);

impl<'a> VersionCmd<'a> {
    /// Creates a new `VersionCmd` with the specified version string.
    ///
    /// # Arguments
    ///
    /// * `str` - A string slice that holds the version number (e.g., "1.0.0").
    ///
    /// # Returns
    ///
    /// A new `VersionCmd` instance containing the provided version string.
    pub fn new(str: &'a str) -> Self {
        VersionCmd(str)
    }

    /// Executes the version command, writing the version to the provided writer.
    ///
    /// The version is formatted with a “v” prefix and a newline suffix.
    ///
    /// # Arguments
    ///
    /// * `w` - Any type that implements the `Write` trait, to which the version will be written.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the write operation was successful.
    /// * `Err(anyhow::Error)` if the write operation failed.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use darkforge_examples::version::VersionCmd;
    ///
    /// let cmd = VersionCmd::new("1.0.0");
    /// let mut buf = Vec::new();
    ///
    /// cmd.run(&mut buf).expect("should have succeeded");
    ///
    /// assert_eq!(String::from_utf8_lossy(buf.as_slice()), "v1.0.0\n");
    /// ```
    pub fn run(&self, mut w: impl Write) -> Result<(), anyhow::Error> {
        writeln!(w, "v{}", self.0).map_err(|e| anyhow!("failed to write version: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn test_prints_version() {
        let cmd = VersionCmd::new("1.0.0");

        let mut buf = Vec::new();
        cmd.run(&mut buf).expect("should have succeeded");

        assert_eq!(String::from_utf8_lossy(buf.as_slice()), "v1.0.0\n");
    }

    #[test]
    fn test_fails_when_write_returns_an_error() {
        struct FailingWriter;
        impl Write for FailingWriter {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::Other, "failed"))
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let cmd = VersionCmd::new("1.0.0");
        let err = cmd.run(FailingWriter).expect_err("should have failed");

        assert_eq!(
            err.to_string(),
            format!("failed to write version: {}", io::Error::new(io::ErrorKind::Other, "failed"))
        );
    }
}
