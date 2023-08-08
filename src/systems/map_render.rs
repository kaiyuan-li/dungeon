use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let p = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let idx = map_idx(p);
            let glyph;
            // For those pixels that's not on the map, just render nothing
            if !Map::in_bounds(p) {
                glyph = to_cp437('#');
            } else {
                glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#')
                }
            }
            draw_batch.set(
                p - offset,
                ColorPair::new(WHITE, BLACK),
                glyph
            );
        }
    }
}