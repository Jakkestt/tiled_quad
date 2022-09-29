use macroquad::{
    color::colors::WHITE,
    math::{vec2, vec3, Rect, Vec2},
    models::{Mesh, Vertex},
    prelude::draw_mesh,
    texture::Texture2D,
};

pub struct QuadMesh(Mesh);

impl QuadMesh {
    pub fn new(texture: &Texture2D) -> Self {
        QuadMesh(Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: Some(*texture),
        })
    }

    /// Add a quad made up of vertices to the mesh.
    pub fn add_quad(&mut self, position: Vec2, size: f32, uv: Rect, inds: u16) -> u16 {
        self.0.vertices.push(Vertex {
            position: vec3(size * position.x, size * position.y, 0.),
            uv: vec2(uv.x, uv.y),
            color: WHITE,
        });
        self.0.vertices.push(Vertex {
            position: vec3(size * position.x, size * position.y, 0.) + vec3(size, 0f32, 0.),
            uv: vec2(uv.x + uv.w, uv.y),
            color: WHITE,
        });
        self.0.vertices.push(Vertex {
            position: vec3(size * position.x, size * position.y, 0.) + vec3(size, size, 0.),
            uv: vec2(uv.x + uv.w, uv.y + uv.h),
            color: WHITE,
        });
        self.0.vertices.push(Vertex {
            position: vec3(size * position.x, size * position.y, 0.) + vec3(0f32, size, 0.),
            uv: vec2(uv.x, uv.y + uv.h),
            color: WHITE,
        });

        self.0.indices.push(inds + 0);
        self.0.indices.push(inds + 1);
        self.0.indices.push(inds + 2);
        self.0.indices.push(inds + 2);
        self.0.indices.push(inds + 3);
        self.0.indices.push(inds + 0);
        inds + 4
    }

    pub fn draw(&self) {
        draw_mesh(&self.0);
    }
}
