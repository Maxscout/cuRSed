/*
This file is part of cuRSed.

cuRSed is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

cuRSed is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with cuRSed. If not, see <https://www.gnu.org/licenses/>.
*/

#![allow(non_snake_case)] // "cuRSed" is considered non snake case so just turn it off
pub mod style;
mod window;

use std::collections::HashMap;
use std::io::ErrorKind;
use std::io::Stdout;
use std::io::Write;

use neutuino::os::RawTerminal;
use style::Style;
use window::Window;
use window::WindowId;

pub struct Terminal {
    raw_terminal: RawTerminal,
    stdout: Stdout,

    windows: HashMap<WindowId, Window>,
}

impl Terminal {
    pub fn new() -> std::io::Result<Self> {
        let mut stdout = std::io::stdout();
        let raw_terminal = RawTerminal::new()?;

        neutuino::os::enable_ansi()?;
        stdout.write(neutuino::ansi::ALT_SCREEN_ENTER.as_bytes())?;

        Ok(Self {
            raw_terminal,
            stdout,
            windows: HashMap::new(),
        })
    }

    pub fn write_string(&mut self, str: impl Into<String>, style: Style) -> std::io::Result<()> {
        self.stdout.write(style.apply().as_bytes())?;
        self.stdout.write(str.into().as_bytes())?;
        Ok(())
    }

    pub fn refresh(&mut self) {}

    pub fn window_from_id(&mut self, window_id: &WindowId) -> std::io::Result<&mut Window> {
        if self.windows.contains_key(window_id) {
            Ok(self.windows.get_mut(window_id).unwrap())
        } else {
            Err(ErrorKind::InvalidInput.into())
        }
    }

    pub fn quit(self) { /* Do nothing (Drop) */
    }
}

impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.stdout
            .write(neutuino::ansi::ALT_SCREEN_EXIT.as_bytes())
            .unwrap();
    }
}
