use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use image::{ImageBuffer, Rgba, ImageError};
use std::{
    io::{self, Write},
    time::Duration,
};
use base64::Engine;

pub struct Graphics {
    stdout: io::Stdout,
    width: u32,
    height: u32,
}

impl Graphics {
    pub fn new() -> io::Result<Self> {
        let (cols, rows) = crossterm::terminal::size()?;
        let width = cols as u32 * 8;  // Approximate pixels per character
        let height = rows as u32 * 16; // Approximate pixels per character
        
        // Enter alternate screen and enable raw mode
        execute!(io::stdout(), EnterAlternateScreen)?;
        crossterm::terminal::enable_raw_mode()?;
        
        // Clear screen and hide cursor
        execute!(io::stdout(), Clear(ClearType::All))?;
        execute!(io::stdout(), Hide)?;

        Ok(Self {
            stdout: io::stdout(),
            width,
            height,
        })
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn move_cursor(&mut self, col: u16, row: u16) -> io::Result<()> {
        execute!(self.stdout, crossterm::cursor::MoveTo(col, row))
    }

    pub fn draw_image(&mut self, img: &ImageBuffer<Rgba<u8>, Vec<u8>>, col: u16, row: u16) -> io::Result<()> {
        // Move cursor to position
        self.move_cursor(col, row)?;

        // Convert image to PNG
        let mut png_data = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut png_data), image::ImageOutputFormat::Png)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let png_base64 = base64::engine::general_purpose::STANDARD.encode(&png_data);

        // Send the image to the terminal using kitty graphics protocol
        self.stdout.write_all(b"\x1b_G")?; // Start graphics command
        self.stdout.write_all(format!("a=T,f=100,s={},v={},c={},r={}", 
            img.width(), img.height(), 
            img.width() / 8, img.height() / 16).as_bytes())?;
        self.stdout.write_all(b";")?;
        self.stdout.write_all(png_base64.as_bytes())?;
        self.stdout.write_all(b"\x1b\\")?; // End graphics command
        self.stdout.flush()
    }

    pub fn draw_text(&mut self, text: &str, col: u16, row: u16) -> io::Result<()> {
        self.move_cursor(col, row)?;
        print!("{}", text);
        self.stdout.flush()
    }

    pub fn wait_for_key(&mut self, key: char) -> io::Result<()> {
        loop {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.kind == KeyEventKind::Press && 
                       key_event.code == KeyCode::Char(key) {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        // Disable raw mode and leave alternate screen
        crossterm::terminal::disable_raw_mode()?;
        execute!(self.stdout, LeaveAlternateScreen)?;
        execute!(self.stdout, Show)
    }
} 