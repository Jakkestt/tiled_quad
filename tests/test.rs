use macroquad::prelude::*;
use tiled_quad::Tile;

#[macroquad::test]
#[test]
async fn test() {
    let map = Tile::from_map("assets/maps/map2.tmx").await;
}
