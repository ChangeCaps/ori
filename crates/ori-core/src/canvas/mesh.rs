use std::{f32::consts::PI, mem, slice};

use crate::{
    image::Texture,
    layout::{Affine, Point, Rect, Vector},
};

use super::{Color, Curve};

/// A vertex in a [`Mesh`].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vertex {
    /// The position of the vertex.
    pub position: Point,
    /// The texture coordinates of the vertex.
    pub tex_coords: Point,
    /// The color of the vertex.
    pub color: Color,
}

impl Vertex {
    /// Create a new vertex with `position` and color`.
    pub fn new_color(position: Point, color: Color) -> Self {
        Self {
            position,
            tex_coords: Point::ZERO,
            color,
        }
    }

    /// Transform the vertex by `transform`.
    pub fn transform(&self, transform: Affine) -> Self {
        Self {
            position: transform * self.position,
            tex_coords: self.tex_coords,
            color: self.color,
        }
    }
}

/// A mesh containing vertices, indices and an optional image.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mesh {
    /// The vertices of the mesh.
    pub vertices: Vec<Vertex>,
    /// The indices of the mesh.
    pub indices: Vec<u32>,
    /// The image of the mesh.
    pub texture: Option<Texture>,
}

impl Mesh {
    /// Create a new empty mesh.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get whether the mesh is empty.
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty() && self.indices.is_empty()
    }

    /// Extend the mesh with the vertices and indices of `other`.
    pub fn extend(&mut self, other: &Self) {
        let offset = self.vertices.len() as u32;
        let offset_indices = other.indices.iter().map(|i| i + offset);

        self.vertices.extend_from_slice(&other.vertices);
        self.indices.extend(offset_indices);
    }

    /// Extend the mesh with the vertices and indices of `other` transformed by `transform`.
    pub fn extend_transformed(&mut self, other: &Self, transform: Affine) {
        self.extend_with(other, |v| v.transform(transform));
    }

    /// Extend the mesh with the vertices and indices of `other` transformed by `transform`,
    /// rounding every position.
    pub fn extend_transformed_pixel_perfect(&mut self, other: &Self, transform: Affine) {
        self.extend_with(other, |mut vertex| {
            vertex = vertex.transform(transform);
            vertex.position = vertex.position.round();
            vertex
        })
    }

    /// Extend the mesh with the vertices and indices of `other` applying `f` to every vertex.
    pub fn extend_with(&mut self, other: &Self, mut f: impl FnMut(Vertex) -> Vertex) {
        let offset = self.vertices.len() as u32;
        let transformed_vertices = other.vertices.iter().map(|v| f(*v));
        let offset_indices = other.indices.iter().map(|i| i + offset);

        self.vertices.extend(transformed_vertices);
        self.indices.extend(offset_indices);
    }

    /// Set the image of the mesh.
    pub fn set_texture(&mut self, image: impl Into<Texture>) {
        self.texture = Some(image.into());
    }

    /// Get the bounds of the mesh.
    pub fn transform(&mut self, transform: Affine) {
        for vertex in &mut self.vertices {
            *vertex = vertex.transform(transform);
        }
    }

    /// Get the bytes of the vertices.
    pub fn vertex_bytes(&self) -> &[u8] {
        let data = self.vertices.as_ptr() as *const u8;
        let len = self.vertices.len() * mem::size_of::<Vertex>();
        unsafe { slice::from_raw_parts(data, len) }
    }

    /// Get the bytes of the indices.
    pub fn index_bytes(&self) -> &[u8] {
        let data = self.indices.as_ptr() as *const u8;
        let len = self.indices.len() * mem::size_of::<u32>();
        unsafe { slice::from_raw_parts(data, len) }
    }

    /// Hit test the mesh.
    ///
    /// Returns true if any of the triangles in the mesh contains the given point.
    pub fn intersects_point(&self, point: Point) -> bool {
        // https://stackoverflow.com/a/2049593
        fn triangle_contains_point(a: Point, b: Point, c: Point, point: Point) -> bool {
            let v0 = c - a;
            let v1 = b - a;
            let v2 = point - a;

            let dot00 = v0.dot(v0);
            let dot01 = v0.dot(v1);
            let dot02 = v0.dot(v2);
            let dot11 = v1.dot(v1);
            let dot12 = v1.dot(v2);

            let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
            let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
            let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

            (u >= 0.0) && (v >= 0.0) && (u + v < 1.0)
        }

        for triangle in self.indices.chunks_exact(3) {
            let a = self.vertices[triangle[0] as usize].position;
            let b = self.vertices[triangle[1] as usize].position;
            let c = self.vertices[triangle[2] as usize].position;

            if triangle_contains_point(a, b, c, point) {
                return true;
            }
        }

        false
    }

    /// Create a circle mesh with the given `center`, `radius`, and `color`.
    pub fn circle(center: Point, radius: f32, color: Color) -> Self {
        let mut mesh = Mesh::new();

        let center = Vertex::new_color(center, color);
        mesh.vertices.push(center);

        let circumference = radius * 2.0 * PI;
        let steps = (circumference / Curve::RESOLUTION).ceil() as usize;

        for i in 0..=steps {
            let angle = i as f32 / steps as f32 * PI * 2.0;
            let x = angle.cos();
            let y = angle.sin();
            let vertex = Vertex::new_color(center.position + Vector::new(x, y) * radius, color);
            mesh.vertices.push(vertex);

            if i < steps {
                mesh.indices.push(0);
                mesh.indices.push(i as u32 + 1);
                mesh.indices.push(i as u32 + 2);
            }
        }

        mesh
    }

    /// Create a rectangle mesh with the given `rect` and `color`.
    pub fn rect(rect: Rect, color: Color) -> Self {
        let mut mesh = Mesh::new();

        let v0 = Vertex {
            position: rect.top_left(),
            tex_coords: Point::new(0.0, 0.0),
            color,
        };
        let v1 = Vertex {
            position: rect.top_right(),
            tex_coords: Point::new(1.0, 0.0),
            color,
        };
        let v2 = Vertex {
            position: rect.bottom_right(),
            tex_coords: Point::new(1.0, 1.0),
            color,
        };
        let v3 = Vertex {
            position: rect.bottom_left(),
            tex_coords: Point::new(0.0, 1.0),
            color,
        };

        mesh.vertices.push(v0);
        mesh.vertices.push(v1);
        mesh.vertices.push(v2);
        mesh.vertices.push(v3);

        mesh.indices.push(0);
        mesh.indices.push(1);
        mesh.indices.push(2);
        mesh.indices.push(0);
        mesh.indices.push(2);
        mesh.indices.push(3);

        mesh
    }
}
