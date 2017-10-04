
use ggez::*;
use ggez::graphics::DrawMode;
use cgmath::Vector2;

use core::iso_coords::IsoCoord;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    sprite: u8,
    free_movement: bool,
}

pub type GridCoordinate = [u32; 2];

#[derive(Debug, Copy, Clone)]
pub enum GridDirection {
    DirectionNorth,
    DirectionSouth,
    DirectionEast,
    DirectionWest,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            sprite: 0,
            free_movement: false,
        }
    }

    fn set_sprite(&mut self, sprite: u8) {
        self.sprite = sprite;
    }

    fn set_free_move(&mut self, b: bool) {
        self.free_movement = b;
    }

    fn is_free_move(&self) -> bool {
        self.free_movement
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
    // TODO: Move out all this, why hold position?
    position: Vector2<f32>,
    tiles: Vec<Tile>,
}

impl Board {
    pub fn new(width: u32, height: u32, tile_size: f32, pos: Vector2<f32>) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();

        // Manually construct a simple square board
        for i in 0..height {
            for j in 0..width {
                let mut new_tile = Tile::new();
                if i % (height - 1) == 0 {
                    new_tile.set_sprite(1);
                    new_tile.set_free_move(true);
                }
                if j == 0 || j == (width - 1) {
                    new_tile.set_sprite(1);
                    new_tile.set_free_move(true);
                }
                tiles.push(new_tile);
            }
        }

        Board {
            width: width,
            height: height,
            tile_size: tile_size,
            position: pos,
            tiles: tiles,
        }
    }

    pub fn get_neighbour(&self,
                         coord: GridCoordinate,
                         direction: GridDirection)
                         -> Option<GridCoordinate> {

        // TODO: Move out bound checks into collision detection.
        //       Pretty ungly shit going on here
        match direction {
            GridDirection::DirectionNorth => {
                if coord[1] == 0 {
                    return None;
                }
                Some([coord[0], coord[1] - 1])
            }
            GridDirection::DirectionSouth => {
                if coord[1] == self.height {
                    return None;
                }
                Some([coord[0], coord[1] + 1])
            }
            GridDirection::DirectionEast => {
                if coord[0] == self.height {
                    return None;
                }
                Some([coord[0] + 1, coord[1]])
            }
            GridDirection::DirectionWest => {
                if coord[0] == 0 {
                    return None;
                }
                Some([coord[0] - 1, coord[1]])
            }
        }
    }

    pub fn get_tile_coordinates(&self, point: Vector2<f32>) -> GridCoordinate {
        [((point.x - self.position.x) / self.tile_size).floor() as u32,
         ((point.y - self.position.y) / self.tile_size).floor() as u32]
    }

    pub fn get_tile_center_world_coordinate(&self, cell: GridCoordinate) -> Vector2<f32> {
        Vector2::new(self.position.x + (cell[0] as f32 + 0.5) * self.tile_size,
                     self.position.y + (cell[1] as f32 + 0.5) * self.tile_size)
    }

    #[allow(dead_code)]
    pub fn get_tile(&self, coord: GridCoordinate) -> &Tile {
        let i = coord[0] + self.width * coord[1];
        &self.tiles[i as usize]
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        for i in 0..self.height {
            for j in 0..self.width {

                // Move coordinate anchor point from middle of rectangle to
                // upper left corner.
                let x = self.position.x + j as f32 * self.tile_size;
                let y = self.position.y + i as f32 * self.tile_size;

                if self.tiles[(i + self.width * j) as usize].get_tile_type() == 0 {
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

                graphics::polygon(ctx, DrawMode::Fill, &rect)?;
            }
        }
        Ok(())
    }
}
