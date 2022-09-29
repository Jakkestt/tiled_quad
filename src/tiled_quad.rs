use crate::util::mesh::QuadMesh;
use crate::util::tilesheet::Tilesheet;
use macroquad::{math::vec2, texture::Texture2D};
use tiled::{FiniteTileLayer, Map};

pub struct Tile {
    layers: Vec<QuadMesh>,
}

impl Tile {
    pub async fn from_map(map: Map) -> Self {
        let tileset = map.tilesets()[0].clone();
        let tilesheet = Tilesheet::from_tileset(tileset).await;

        let layers = map
            .layers()
            .filter_map(|layer| match &layer.layer_type() {
                tiled::LayerType::TileLayer(l) => Some(generate_mesh(
                    match l {
                        tiled::TileLayer::Finite(f) => f,
                        tiled::TileLayer::Infinite(_) => panic!("Infinite maps not supported"),
                    },
                    &tilesheet,
                    tilesheet.texture(),
                )),
                _ => None,
            })
            .collect();

        Self { layers }
    }

    pub fn draw(&self) {
        for mesh in self.layers.iter() {
            mesh.draw();
        }
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
