use std::sync::Arc;

use macroquad::{math::Rect, texture};

use tiled::Tileset;

/// A container for a tileset and the texture it references.
pub struct Tilesheet {
    texture: texture::Texture2D,
    tileset: Arc<Tileset>,
}

impl Tilesheet {
    /// Create a tilesheet from a Tiled tileset, loading its texture along the way.
    pub async fn from_tileset(tileset: Arc<Tileset>) -> Self {
        let tileset_image = tileset.image.as_ref().unwrap();

        let texture_path = &tileset_image
            .source
            .to_str()
            .expect("obtaining valid UTF-8 path");
        println!("{:?}", texture_path);
        let texture = texture::load_texture(texture_path).await.unwrap();

        Tilesheet { texture, tileset }
    }

    pub fn texture(&self) -> &texture::Texture2D {
        &self.texture
    }

    pub fn tile_rect(&self, id: u32) -> Rect {
        let tile_width = self.tileset.tile_width;
        let tile_height = self.tileset.tile_height;
        let spacing = self.tileset.spacing;
        let margin = self.tileset.margin;
        let tiles_per_row =
            (self.texture.width() as u32 - margin + spacing) / (tile_width + spacing);
        let pixel_size_x = 1. / self.texture.width() as f32;
        let pixel_size_y = 1. / self.texture.height() as f32;
        let x = id % tiles_per_row * tile_width;
        let y = id / tiles_per_row * tile_height;

        Rect {
            x: x as f32 * pixel_size_x,
            y: y as f32 * pixel_size_y,
            w: tile_width as f32 * pixel_size_x,
            h: tile_height as f32 * pixel_size_y,
        }
    }
}
