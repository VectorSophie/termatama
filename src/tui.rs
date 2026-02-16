use crossterm::{
    cursor,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Write};

pub struct TuiRenderer {
    stdout: std::io::Stdout,
}

impl TuiRenderer {
    pub fn new() -> std::io::Result<Self> {
        let mut stdout = stdout();
        terminal::enable_raw_mode()?;
        stdout.execute(terminal::EnterAlternateScreen)?;
        stdout.execute(cursor::Hide)?;
        Ok(Self { stdout })
    }

    pub fn render(&mut self, lcd: &[[bool; 32]; 16]) -> std::io::Result<()> {
        self.stdout.queue(cursor::MoveTo(0, 0))?;

        self.stdout
            .queue(Print("┌────────────────────────────────┐\r\n"))?;

        for row in lcd {
            self.stdout.queue(Print("│"))?;
            for &pixel in row {
                if pixel {
                    self.stdout.queue(Print("█"))?;
                } else {
                    self.stdout.queue(Print(" "))?;
                }
            }
            self.stdout.queue(Print("│\r\n"))?;
        }

        self.stdout
            .queue(Print("└────────────────────────────────┘\r\n"))?;

        self.stdout.flush()?;
        Ok(())
    }
}

impl Drop for TuiRenderer {
    fn drop(&mut self) {
        let _ = self.stdout.execute(cursor::Show);
        let _ = self.stdout.execute(terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}
