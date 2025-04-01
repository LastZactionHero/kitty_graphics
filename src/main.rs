mod graphics;
mod elements;

use elements::Grid;
use image::Rgba;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut graphics = graphics::Graphics::new()?;
    
    // Create a 16x16 grid with 16x16 pixel squares
    let grid = Grid::new(
        16,  // rows
        16,  // cols
        16,  // square width
        16,  // square height
        0,   // x offset
        0,   // y offset
        Rgba([0, 255, 255, 255]), // neon blue color
    );

    // Draw the grid at position (0, 0)
    graphics.draw_image(&grid.render(), 0, 0)?;

    // Wait for 'q' key
    graphics.wait_for_key('q')?;

    // Cleanup
    graphics.cleanup()?;
    Ok(())
}
