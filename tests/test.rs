use macroquad::prelude::*;
use tiled_quad::Tile;

#[macroquad::test]
#[test]
async fn test() {
    let mut x = 0.;
    let mut y = 0.;
    let mut draw_collision = true;
    let map = Tile::from_map("assets/maps/map2.tmx").await;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::C) {
            draw_collision = !draw_collision;
        }
        if is_key_down(KeyCode::W) {
            y -= 10.;
        }
        if is_key_down(KeyCode::S) {
            y += 10.;
        }
        if is_key_down(KeyCode::A) {
            x -= 10.;
        }
        if is_key_down(KeyCode::D) {
            x += 10.;
        }
        clear_background(WHITE);
        set_camera(&Camera2D {
            zoom: vec2(0.01, screen_width() / screen_height() * -0.01),
            target: vec2(x, y),
            ..Default::default()
        });
        map.draw_tile_layers();
        if draw_collision {
            map.draw_object_layers();
        }
        set_default_camera();
        next_frame().await;
    }
}
