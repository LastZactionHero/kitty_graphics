mod graphics;
mod elements;

use crate::graphics::Graphics;
use crate::elements::{Grid, BoxShape, Label};
use image::Rgba;
use crossterm::event::{self, KeyCode, KeyEventKind, Event};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mut graphics = Graphics::new()?;
    let (width, height) = graphics.dimensions();
    
    // Create a 16x16 grid with 32x32 pixel cells
    let mut grid = Grid::new(
        16,  // rows
        16,  // cols
        32,  // square width
        32,  // square height
        0,   // x offset
        0,   // y offset
        Rgba([0, 255, 255, 255]), // neon blue color
    );
    
    // Set initial active cell
    grid.set_active_cell(0, 0);
    
    // Draw the grid at the top-left corner
    let grid_image = grid.render();
    graphics.draw_image(&grid_image, 0, 0)?;
    
    // Main event loop
    let mut current_pos = (0, 0);
    let grid_id = 1; // We know this is the first image we drew
    
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up => {
                            if current_pos.1 > 0 {
                                current_pos.1 -= 1;
                                grid.set_active_cell(current_pos.0, current_pos.1);
                                let grid_image = grid.render();
                                graphics.draw_image(&grid_image, 0, 0)?;
                            }
                        }
                        KeyCode::Down => {
                            if current_pos.1 < 15 {
                                current_pos.1 += 1;
                                grid.set_active_cell(current_pos.0, current_pos.1);
                                let grid_image = grid.render();
                                graphics.draw_image(&grid_image, 0, 0)?;
                            }
                        }
                        KeyCode::Left => {
                            if current_pos.0 > 0 {
                                current_pos.0 -= 1;
                                grid.set_active_cell(current_pos.0, current_pos.1);
                                let grid_image = grid.render();
                                graphics.draw_image(&grid_image, 0, 0)?;
                            }
                        }
                        KeyCode::Right => {
                            if current_pos.0 < 15 {
                                current_pos.0 += 1;
                                grid.set_active_cell(current_pos.0, current_pos.1);
                                let grid_image = grid.render();
                                graphics.draw_image(&grid_image, 0, 0)?;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    graphics.cleanup()?;
    Ok(())
}
