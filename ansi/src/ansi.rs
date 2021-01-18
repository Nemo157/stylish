use crate::util;
use stylish_core::{Result, Style, Write};

#[derive(Clone, Debug, Default)]
pub struct Ansi<T: core::fmt::Write> {
    inner: T,
    current: Option<Style>,
}

impl<T: core::fmt::Write> Ansi<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            current: None,
        }
    }

    pub fn finish(mut self) -> Result<T> {
        if self.current.is_some() {
            self.inner.write_str("\x1b[0m")?;
        }
        Ok(self.inner)
    }
}

impl<T: core::fmt::Write> Write for Ansi<T> {
    fn write_str(&mut self, s: &str, style: Style) -> Result {
        let diff = style.diff_from(self.current.unwrap_or_default());
        let segments = [
            diff.foreground.map(util::foreground),
            diff.background.map(util::background),
            diff.intensity.map(util::intensity),
        ];
        let mut segments = segments.iter().filter_map(|&s| s);
        if let Some(segment) = segments.next() {
            self.inner.write_str("\x1b[")?;
            self.inner.write_str(segment)?;
            for segment in segments {
                self.inner.write_str(";")?;
                self.inner.write_str(segment)?;
            }
            self.inner.write_str("m")?;
        }
        self.current = Some(style);
        write!(self.inner, "{}", s)?;
        Ok(())
    }
}
