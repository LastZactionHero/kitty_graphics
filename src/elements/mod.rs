use image::{ImageBuffer, Rgba};
use crate::graphics::Graphics;
use std::io;

pub struct Line {
    width: u32,
    height: u32,
    color: Rgba<u8>,
}

impl Line {
    pub fn new(width: u32, height: u32, color: Rgba<u8>) -> Self {
        Self {
            width,
            height,
            color,
        }
    }

    pub fn horizontal(width: u32, color: Rgba<u8>) -> Self {
        Self::new(width, 1, color)
    }

    pub fn vertical(height: u32, color: Rgba<u8>) -> Self {
        Self::new(1, height, color)
    }

    pub fn render(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut img = ImageBuffer::new(self.width, self.height);
        
        // Fill with color
        for pixel in img.pixels_mut() {
            *pixel = self.color;
        }
        
        img
    }
}

#[derive(Debug)]
pub struct BoxShape {
    width: u32,
    height: u32,
    color: Rgba<u8>,
}

impl BoxShape {
    pub fn new(width: u32, height: u32, color: Rgba<u8>) -> Self {
        Self {
            width,
            height,
            color,
        }
    }

    pub fn render(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut img = ImageBuffer::new(self.width, self.height);
        
        // Fill with transparent background
        for pixel in img.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 0]);
        }

        // Draw borders
        for x in 0..self.width {
            img.put_pixel(x, 0, self.color); // Top border
            img.put_pixel(x, self.height - 1, self.color); // Bottom border
        }
        for y in 0..self.height {
            img.put_pixel(0, y, self.color); // Left border
            img.put_pixel(self.width - 1, y, self.color); // Right border
        }
        
        img
    }
}

pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn render(&self, graphics: &mut Graphics, col: u16, row: u16) -> io::Result<()> {
        graphics.draw_text(&self.text, col, row)
    }
}

#[derive(Debug)]
pub struct Grid {
    rows: u32,
    cols: u32,
    square_width: u32,
    square_height: u32,
    x_offset: u32,
    y_offset: u32,
    color: Rgba<u8>,
}

impl Grid {
    pub fn new(
        rows: u32,
        cols: u32,
        square_width: u32,
        square_height: u32,
        x_offset: u32,
        y_offset: u32,
        color: Rgba<u8>,
    ) -> Self {
        Self {
            rows,
            cols,
            square_width,
            square_height,
            x_offset,
            y_offset,
            color,
        }
    }

    pub fn render(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let total_width = self.cols * self.square_width;
        let total_height = self.rows * self.square_height;
        let mut img = ImageBuffer::new(total_width, total_height);

        // Fill with transparent background
        for pixel in img.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 0]);
        }

        // Draw vertical lines
        for col in 0..self.cols {
            let x = col * self.square_width;
            for y in 0..total_height {
                img.put_pixel(x, y, self.color);
            }
        }
        // Draw final vertical line
        for y in 0..total_height {
            img.put_pixel(total_width - 1, y, self.color);
        }

        // Draw horizontal lines
        for row in 0..self.rows {
            let y = row * self.square_height;
            for x in 0..total_width {
                img.put_pixel(x, y, self.color);
            }
        }
        // Draw final horizontal line
        for x in 0..total_width {
            img.put_pixel(x, total_height - 1, self.color);
        }

        img
    }
} 