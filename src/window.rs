use std::io::{ErrorKind, Write};

use crate::Terminal;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct WindowId(String);

pub trait ToWindowId {
    fn to_window_id(self) -> WindowId;
}

impl<T: Into<String>> ToWindowId for T {
    fn to_window_id(self) -> WindowId {
        WindowId(self.into())
    }
}

pub struct WindowOptions {
    pub title: String,
    pub border: bool,
    pub wrap_contents: bool,

    pub width: u16,
    pub height: u16,

    pub posx: u16,
    pub posy: u16,
}

pub struct Window {
    pub options: WindowOptions,
    pub(crate) contents: String,
    last_char_written: (u16, u16),
}

impl Window {
    /// Creates a window from an id in a terminal.
    pub fn new(
        terminal: &mut Terminal,
        id: WindowId,
        window_options: WindowOptions,
    ) -> std::io::Result<WindowId> {
        if !terminal.windows.contains_key(&id) {
            terminal.windows.insert(
                id.clone(),
                Self {
                    options: window_options,
                    contents: String::new(),
                    last_char_written: (0, 0),
                },
            );
            Ok(id)
        } else {
            Err(ErrorKind::AlreadyExists.into())
        }
    }

    pub fn write_at_pos(&mut self, pos: (u16, u16), buf: &[u8]) -> std::io::Result<usize> {
        let mut row_counter = 0;
        let mut char_amount = 0;
        for char in self.contents.chars() {
            if row_counter == pos.1 {
                break;
            }

            if char == '\n' {
                row_counter += 1
            }

            char_amount += 1;
        }

        let insert = String::from_utf8(buf.to_vec()).unwrap_or(String::new());

        self.contents
            .insert_str((char_amount + pos.0) as usize, &insert);

        Ok(insert.len())
    }
}

impl Write for Window {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_at_pos(self.last_char_written, buf)
    }

    /// Use Terminal::refresh to flush windows.
    fn flush(&mut self) -> std::io::Result<()> {
        Err(ErrorKind::Unsupported.into())
    }
}
