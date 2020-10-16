use std::{fmt::Display, fs::File, fs::OpenOptions, io::Stdout, io::Write, path::Path};

use crossterm::{
    cursor::Hide,
    cursor::Show,
    terminal::{Clear, ClearType},
    QueueableCommand,
};

pub struct TiwiWriter {
    file: Option<File>,
    stdout: Stdout,
    queued_newline: bool,
}

impl TiwiWriter {
    pub fn stdout() -> Self {
        Self {
            file: None,
            stdout: std::io::stdout(),
            queued_newline: false,
        }
    }

    pub fn stdout_and_file(path: &Path) -> anyhow::Result<TiwiWriter> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)?;

        let stdout = std::io::stdout();

        Ok(Self {
            stdout,
            file: Some(file),
            queued_newline: false,
        })
    }
}

impl TiwiWriter {
    pub fn update<T: Display>(&mut self, value: T) -> anyhow::Result<()> {
        self.newline()?;

        let string = format!("{}", value);

        self.stdout.write("\r".as_bytes())?;
        self.stdout.write(string.as_bytes())?;
        self.stdout.queue(Clear(ClearType::UntilNewLine))?;
        self.stdout.flush()?;

        Ok(())
    }

    pub fn complete<T: Display>(&mut self, value: T) -> anyhow::Result<()> {
        self.newline()?;

        self.update(value)?;
        self.stdout.queue(Hide)?;
        self.stdout.flush()?;

        Ok(())
    }

    pub fn flush<T: Display>(&mut self, value: T) -> anyhow::Result<()> {
        self.newline()?;
        self.update(&value)?;

        if let Some(ref mut file) = self.file {
            let string = format!("{}", value);
            file.write(string.as_bytes())?;
            file.write("\n".as_bytes())?;
            file.flush()?;
        }

        self.stdout.queue(Hide)?;
        self.stdout.flush()?;

        self.queued_newline = true;

        Ok(())
    }

    fn newline(&mut self) -> anyhow::Result<()> {
        if self.queued_newline {
            self.stdout.write("\r\n".as_bytes())?;
            self.stdout.queue(Show)?;
            self.queued_newline = false;
        }

        Ok(())
    }
}

impl Write for TiwiWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(file) = &mut self.file {
            file.write(buf)?;
        }

        self.stdout.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(file) = &mut self.file {
            file.flush()?;
        }

        self.stdout.flush()
    }
}
