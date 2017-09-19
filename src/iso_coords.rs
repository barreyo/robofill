
use ggez::graphics::Point;

#[derive(Debug, Copy, Clone)]
pub struct IsoCoord {
    pub x: f32,
    pub y: f32,
}

impl IsoCoord {
    /// Construct from coordinates in Isometric space.
    pub fn new(x: f32, y: f32) -> Self {
        IsoCoord { x: x, y: y }
    }

    /// Construct a point in Isometric space from cartesian coordinates.
    pub fn from_point(p: Point) -> Self {
        let iso_coords = to_iso(p);

        IsoCoord {
            x: iso_coords.x,
            y: iso_coords.y,
        }
    }

    /// Take cartesian coordinates and covert to isometric coordinate space.
    pub fn from_cartesian(x: f32, y: f32) -> Self {
        let iso_coords = to_iso(Point::new(x, y));

        IsoCoord {
            x: iso_coords.x,
            y: iso_coords.y,
        }
    }

    /// Return the isometric coordinates as a Point.
    pub fn as_point(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

/// From cartesian coordinates to isometric coordinates.
fn to_iso(pt: Point) -> Point {
    let mut p: Point = Point::new(0.0, 0.0);
    p.x = pt.x - pt.y;
    p.y = (pt.x + pt.y) / 2.0;
    p
}
