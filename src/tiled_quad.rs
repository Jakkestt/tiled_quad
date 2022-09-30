use crate::util::mesh::QuadMesh;
use crate::util::tilesheet::Tilesheet;
use macroquad::{
    math::vec2,
    prelude::{draw_rectangle, WHITE},
    texture::Texture2D,
};
use tiled::{FiniteTileLayer, Loader, ObjectData, ObjectShape};

pub struct Tile {
    tile_layers: Vec<QuadMesh>,
    object_layers: Vec<ObjectData>,
}

impl Tile {
    pub async fn from_map(path: &str) -> Self {
        let mut loader = Loader::new();
        let map = loader.load_tmx_map(path).unwrap();
        let tileset = map.tilesets()[0].clone();
        let tilesheet = Tilesheet::from_tileset(tileset).await;

        let mut tile_layers = Vec::new();
        let mut object_layers = Vec::new();
        for layer in map.layers() {
            match &layer.layer_type() {
                tiled::LayerType::TileLayer(l) => Some(tile_layers.push(generate_mesh(
                    match l {
                        tiled::TileLayer::Finite(f) => f,
                        tiled::TileLayer::Infinite(_) => panic!("Infinite maps not supported"),
                    },
                    &tilesheet,
                    tilesheet.texture(),
                ))),
                tiled::LayerType::ObjectLayer(l) => Some(for object in l.object_data() {
                    object_layers.push(object.clone());
                }),
                _ => None,
            };
        }

        Self {
            tile_layers,
            object_layers,
        }
    }

    pub fn draw_tile_layers(&self) {
        for mesh in self.tile_layers.iter() {
            mesh.draw();
        }
    }

    pub fn draw_object_layers(&self) {
        for object in &self.object_layers {
            match object.shape {
                ObjectShape::Rect { width, height } => {
                    draw_rectangle(object.x, object.y, width, height, WHITE)
                }
                _ => {}
            }
        }
    }

    pub fn get_objects(&self) -> &Vec<ObjectData> {
        &self.object_layers
    }
}

fn generate_mesh(layer: &FiniteTileLayer, tilesheet: &Tilesheet, texture: &Texture2D) -> QuadMesh {
    let (width, height) = (layer.width() as usize, layer.height() as usize);
    let mut mesh = QuadMesh::new(texture);
    let mut inds = 0;
    for x in 0..width as i32 {
        for y in 0..height as i32 {
            if let Some(tile) = layer.get_tile(x, y) {
                let uv = tilesheet.tile_rect(tile.id());
                inds = mesh.add_quad(vec2(x as f32, y as f32), 16., uv, inds);
            }
        }
    }

    mesh
}
