use std::io::Write;

pub struct WindowOptions {
    title: String,
    border: bool,
    wrap_contents: bool,
}

pub struct Window {
    pub options: WindowOptions,
    pub width: u32,
    pub height: u32,
    pub posx: u32,
    pub posy: u32,
    pub contents: String,
}

impl Window {
    pub fn refresh(&self) -> std::io::Result<()> {
        // Do nothing
        Ok(())
    }
}

impl Write for Window {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.contents += std::str::from_utf8(buf).unwrap();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.refresh()
    }
}
