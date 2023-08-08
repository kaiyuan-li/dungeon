use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(p: Point) -> usize {
    (p.y * SCREEN_WIDTH + p.x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                let p = Point::new(x, y);
                let idx = map_idx(p);
                let glyph;
                // For those pixels that's not on the map, just render nothing
                if !Self::in_bounds(p) {
                    glyph = to_cp437('#');
                } else {
                    glyph = match self.tiles[idx] {
                        TileType::Floor => to_cp437('.'),
                        TileType::Wall => to_cp437('#')
                    }
                }
                ctx.set(x - camera.left_x, y - camera.top_y, WHITE, BLACK, glyph);
            }
        }
    }

    // Whether a point is in the map
    pub fn in_bounds(point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        Self::in_bounds(point) && self.tiles[map_idx(point)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !Self::in_bounds(point) {
            None
        } else {
            Some(map_idx(point))
        }
    }
}
