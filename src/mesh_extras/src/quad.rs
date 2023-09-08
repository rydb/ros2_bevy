//use super::{Indices, Mesh};
use bevy::prelude::Mesh;
use bevy::render::mesh::*;
use glam::*;
use bevy::render::mesh::Indices;

/// quad that corresponds to a face, and other convienience methods. This is converted to triangles for bevy mesh initialization. 
pub struct Quad {
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
    d: [f32; 3],
}

impl From<(
    [f32; 3], [f32; 3], [f32; 3], [f32; 3]
)> for Quad {
    fn from(value: ([f32; 3], [f32; 3], [f32; 3], [f32; 3]) ) -> Self {
        Self {
            a: value.0,
            b: value.1,
            c: value.2,
            d: value.3,
        }

    }
}

/// (TODO): implement some kind of algorithm for this?
pub fn quads_to_uvs(quads: &Vec<Quad>) -> Vec<[f32; 2]> {
    
    let mut uvs_list = Vec::new();
    
    for _quad in quads {
        // uvs based on Box `Front` UVs, this is going to need a proper algorithm 
        // at some point....
        uvs_list.push([0., 0.]);
        uvs_list.push([1.0, 0.]);
        uvs_list.push([1.0, 1.0]);
        uvs_list.push([0., 1.0]);
    }
    println!("uv count is {:#?}", uvs_list.len());
    return uvs_list

}


pub fn quads_to_normals(quads: &Vec<Quad>) -> Vec<[f32; 3]> {

    let mut normals_list = Vec::new();

    for quad in quads.iter() {
        normals_list.push(Vec3::normalize(Vec3::from(quad.a)).into());
        normals_list.push(Vec3::normalize(Vec3::from(quad.b)).into());
        normals_list.push(Vec3::normalize(Vec3::from(quad.c)).into());
        normals_list.push(Vec3::normalize(Vec3::from(quad.d)).into());

    }     
    println!("normal count is {:#?}", normals_list.len());
    return normals_list
} 

/// take a quad, and get indicies so bevy can turn it into triangles
pub fn quads_to_indices(quads: &Vec<Quad>) -> Indices {
    let mut vertex_indexes = Vec::new();
    let mut i: u32 = 0;
    while i < ((quads.len() * 4) as u32) {
        //println!("quads length is {:#?}", quads.len());
        vertex_indexes.push(i);
        vertex_indexes.push(i + 1);
        vertex_indexes.push(i + 1 + 1);
        vertex_indexes.push(i + 1 + 1);
        vertex_indexes.push(i + 1 + 1 + 1);
        vertex_indexes.push(i);

        i += 4;
    }
    println!("vertex index count is {:#?}" ,vertex_indexes.len());
    return Indices::U32(vertex_indexes);

}

pub fn quads_to_vertexes(quads: &Vec<Quad>) -> Vec<[f32; 3]> {

    let mut position_list = Vec::new();
    for quad in quads {
        position_list.push(quad.a);
        position_list.push(quad.b);
        position_list.push(quad.c);
        position_list.push(quad.d);
    }
    println!("vertexes count is {:#?}", position_list.len());
    return position_list;
}

/// An axis-aligned box defined by its minimum and maximum point.
#[derive(Debug, Copy, Clone)]
pub struct Box {
    pub min_x: f32,
    pub max_x: f32,

    pub min_y: f32,
    pub max_y: f32,

    pub min_z: f32,
    pub max_z: f32,
}

impl Box {
    /// Creates a new box centered at the origin with the supplied side lengths.
    pub fn new(x_length: f32, y_length: f32, z_length: f32) -> Box {
        Box {
            max_x: x_length / 2.0,
            min_x: -x_length / 2.0,
            max_y: y_length / 2.0,
            min_y: -y_length / 2.0,
            max_z: z_length / 2.0,
            min_z: -z_length / 2.0,
        }
    }

    /// Creates a new box given the coordinates of two opposing corners.
    pub fn from_corners(a: Vec3, b: Vec3) -> Box {
        let max = a.max(b);
        let min = a.min(b);
        Box {
            max_x: max.x,
            min_x: min.x,
            max_y: max.y,
            min_y: min.y,
            max_z: max.z,
            min_z: min.z,
        }
    }
}

impl Default for Box {
    fn default() -> Self {
        Box::new(2.0, 1.0, 1.0)
    }
}

impl From<Box> for Mesh {
    fn from(sp: Box) -> Self {
        // suppose Y-up right hand, and camera look from +z to -z
        let faces: Vec<Quad> = vec![
            // Front
            (
            ([sp.min_x, sp.min_y, sp.max_z]),
            ([sp.max_x, sp.min_y, sp.max_z]),
            ([sp.max_x, sp.max_y, sp.max_z]),
            ([sp.min_x, sp.max_y, sp.max_z]),
            ).into(),
            // Back
            (
            ([sp.min_x, sp.max_y, sp.min_z]),
            ([sp.max_x, sp.max_y, sp.min_z]),
            ([sp.max_x, sp.min_y, sp.min_z]),
            ([sp.min_x, sp.min_y, sp.min_z]),
            ).into(),
            // Right
            (
            ([sp.max_x, sp.min_y, sp.min_z]),
            ([sp.max_x, sp.max_y, sp.min_z]),
            ([sp.max_x, sp.max_y, sp.max_z]),
            ([sp.max_x, sp.min_y, sp.max_z]),
            ).into(),
            // Left
            (
            ([sp.min_x, sp.min_y, sp.max_z]),
            ([sp.min_x, sp.max_y, sp.max_z]),
            ([sp.min_x, sp.max_y, sp.min_z]),
            ([sp.min_x, sp.min_y, sp.min_z]),
            ).into(),
            // Top
            (
            ([sp.max_x, sp.max_y, sp.min_z]),
            ([sp.min_x, sp.max_y, sp.min_z]),
            ([sp.min_x, sp.max_y, sp.max_z]),
            ([sp.max_x, sp.max_y, sp.max_z]),
            ).into(),
            // Bottom
            (
            ([sp.max_x, sp.min_y, sp.max_z]),
            ([sp.min_x, sp.min_y, sp.max_z]),
            ([sp.min_x, sp.min_y, sp.min_z]),
            ([sp.max_x, sp.min_y, sp.min_z]),
            ).into(),
        ];

        let positions: Vec<_> = quads_to_vertexes(&faces);//vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = quads_to_normals(&faces);
        let uvs: Vec<_> = quads_to_uvs(&faces);
        let indices = quads_to_indices(&faces);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));
        mesh
    }
}