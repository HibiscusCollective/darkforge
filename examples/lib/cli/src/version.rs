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
use std::{io::Write, process::ExitCode};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version(mut w: impl Write) -> ExitCode {
    let _ = write!(w, "v{VERSION}");
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prints_version() {
        let mut buf = Vec::new();

        let code = version(&mut buf);

        assert_eq!(code, ExitCode::SUCCESS);
        assert_eq!(String::from_utf8_lossy(buf.as_slice()), env!("CARGO_PKG_VERSION"));
    }
}
