use itertools::Itertools;
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

pub fn calculate_face_normals(quads: Vec<Quad>) -> Vec<[f32; 3]> {

    let mut normals_list = Vec::new();

    for quad in quads.iter() {
        let normal = Vec3::normalize(Vec3::from(quad.a)).into();
        normals_list.push(normal);
    }
    return normals_list
} 

/// take a quad, and get indicies so bevy can turn it into triangles
pub fn get_triangle_indices(quads: Vec<Quad>) -> Indices {
    let vertex_indexes = Vec::new();

}

pub fn main() {
    //let sample_quad: Quad = ().into();
    // let sample_quad = Quad {
    //     a: (([0.0, 1.0, 2.0]))
    // }
    let shape: Vec<Quad> = vec![
        //Top
        (
            ([0.0, 1.0, 2.0]),
            ([0.0, 1.0, 2.0]),
            ([0.0, 1.0, 2.0]),
            ([0.0, 1.0, 2.0]),
        ).into(),
        //bottom
        (
            ([0.0, 1.0, 2.0]),
            ([0.0, 1.0, 2.0]),
            ([0.0, 1.0, 2.0]),
            ([0.0, 1.0, 2.0]),
        ).into()
        
    ];
    // let faces: Vec<Quad> = vec![
    // (
    //     //Top
    //     (([0.0, 1.0, 2.0]), ([3.0, 4.0, 5.0]), ([0.0, 1.0, 2.0])),
    //     (([0.0, 1.0, 2.0]), ([3.0, 4.0, 5.0]), ([0.0, 1.0, 2.0])),
    //     (([0.0, 1.0, 2.0]), ([3.0, 4.0, 5.0]), ([0.0, 1.0, 2.0])),
    //     (([0.0, 1.0, 2.0]), ([3.0, 4.0, 5.0]), ([0.0, 1.0, 2.0])),
    // ).into()
    // ];
        
        //[([0.0, 1.0, 2.0]), ([3.0, 4.0, 5.0]), ([0.0, 1.0, 2.0]), ([0.0, 1.0, 2.0]), ([0.0, 1.0, 2.0]), ([0.0, 1.0, 2.0])];
    //let faces: Vec<[[F32; 3]]> = Vec::with_capacity(3);
    //let faces = vertices.iter().map(|arr| arr.iter().map(|cool| cool));
    //let faces = array::IntoIter::vertices.iter().collect::<Vec<[f32; 3]>>(); //<-- broken
}
