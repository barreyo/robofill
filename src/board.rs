
use ggez::*;
use ggez::graphics::DrawMode;

use meta::iso_coords::IsoCoord;

#[derive(Debug, Copy, Clone)]
struct Cell {
    sprite: u8,
}

impl Cell {
    fn new() -> Cell {
        Cell { sprite: 0 }
    }

    fn set_sprite(&mut self, sprite: u8) {
        self.sprite = sprite;
    }

    fn get_tile_type(&self) -> u8 {
        self.sprite
    }
}

#[derive(Debug)]
pub struct Board {
    width: u32,
    height: u32,
    tile_size: f32,
    cells: Vec<Cell>,
}

impl Board {
    pub fn new(width: u32, height: u32, tile_size: f32) -> Self {
        let mut tiles: Vec<Cell> = Vec::new();

        // Manually construct a simple square board
        for i in 0..height {
            for j in 0..width {
                let mut new_cell = Cell::new();
                if i % (height - 1) == 0 {
                    new_cell.set_sprite(1);
                }
                if j == 0 || j == (width - 1) {
                    new_cell.set_sprite(1);
                }
                tiles.push(new_cell);
            }
        }

        Board {
            width: width,
            height: height,
            tile_size: tile_size,
            cells: tiles,
        }
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        for i in 0..self.height {
            for j in 0..self.width {

                // Move coordinate anchor point from middle of rectangle to
                // upper left corner.
                let x = 300.0 + j as f32 * self.tile_size;
                let y = -100.0 + i as f32 * self.tile_size;

                if self.cells[(i + self.width * j) as usize].get_tile_type() == 0 {
                    graphics::set_color(ctx, graphics::Color::new(0.2, 0.99, 0.56, 1.0))?;
                } else {
                    graphics::set_color(ctx, graphics::Color::new(0.73, 0.88, 0.06, 1.0))?;
                }

                // Create a rectangle and transform the coordinates into iso
                let rect = vec![IsoCoord::from_cartesian(x, y).as_point(),
                                IsoCoord::from_cartesian(x + self.tile_size, y).as_point(),
                                IsoCoord::from_cartesian(x + self.tile_size, y + self.tile_size)
                                    .as_point(),
                                IsoCoord::from_cartesian(x, y + self.tile_size).as_point()]
                    .into_boxed_slice();

                graphics::polygon(ctx, DrawMode::Line, &rect)?;
            }
        }
        Ok(())
    }
}
