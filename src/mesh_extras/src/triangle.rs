//use super::{Indices, Mesh};
use bevy::prelude::Mesh;
use bevy::render::mesh::*;
use glam::*;
use bevy::render::mesh::Indices;


/// triangle that coresponds to a face, and other convienience methods.
pub struct Triangle {
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
}

impl From<(
    [f32; 3], [f32; 3], [f32; 3]
)> for Triangle {
    fn from(value: ([f32; 3], [f32; 3], [f32; 3]) ) -> Self {
        Self {
            a: value.0,
            b: value.1,
            c: value.2,
        }

    }
}

pub fn triangles_to_vertexes(quads: &Vec<Triangle>) -> Vec<[f32; 3]> {

    let mut position_list = Vec::new();
    for quad in quads {
        position_list.push(quad.a);
        position_list.push(quad.b);
        position_list.push(quad.c);
    }
    println!("vertexes count is {:#?}", position_list.len());
    return position_list;
}


/// take a quad, and get indicies so bevy can turn it into triangles
pub fn triangles_to_indices(quads: &Vec<Triangle>) -> Indices {
    let mut vertex_indexes = Vec::new();
    let mut i: u32 = 0;
    while i < ((quads.len() * 4) as u32) {
        //println!("quads length is {:#?}", quads.len());
        vertex_indexes.push(i);
        vertex_indexes.push(i + 1);
        vertex_indexes.push(i + 1 + 1);
        // vertex_indexes.push(i + 1 + 1);
        // vertex_indexes.push(i + 1 + 1 + 1);
        // vertex_indexes.push(i);

        i += 4;
    }
    println!("vertex index count is {:#?}" ,vertex_indexes.len());
    return Indices::U32(vertex_indexes);

}

fn face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let (a, b, c) = (Vec3::from(a), Vec3::from(b), Vec3::from(c));
    (b - a).cross(c - a).normalize().into()
}


/// calculates normals for mesh based on indicies. Taken from:
/// https://github.com/bevyengine/bevy/blob/ef27fa89cd593a5a6432ecd339e12d0e3cad3f0f/crates/bevy_render/src/mesh/mesh/mod.rs
pub fn compute_normals_for_mesh(mesh: &mut Mesh) {
    assert!(
        matches!(mesh.primitive_topology(), PrimitiveTopology::TriangleList),
        "`compute_normals` can only work on `TriangleList`s"
    );

    let positions = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .unwrap()
        .as_float3()
        .expect("`Mesh::ATTRIBUTE_POSITION` vertex attributes should be of type `float3`");

    match mesh.indices() {
        Some(indices) => {
            let mut count: usize = 0;
            let mut corners = [0_usize; 3];
            let mut normals = vec![[0.0f32; 3]; positions.len()];
            let mut adjacency_counts = vec![0_usize; positions.len()];

            for i in indices.iter() {
                corners[count % 3] = i;
                count += 1;
                if count % 3 == 0 {
                    let normal = face_normal(
                        positions[corners[0]],
                        positions[corners[1]],
                        positions[corners[2]],
                    );
                    for corner in corners {
                        normals[corner] =
                            (Vec3::from(normal) + Vec3::from(normals[corner])).into();
                        adjacency_counts[corner] += 1;
                    }
                }
            }

            // average (smooth) normals for shared vertices...
            // TODO: support different methods of weighting the average
            for i in 0..normals.len() {
                let count = adjacency_counts[i];
                if count > 0 {
                    normals[i] = (Vec3::from(normals[i]) / (count as f32)).normalize().into();
                }
            }

            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        }
        None => {
            let normals: Vec<_> = positions
                .chunks_exact(3)
                .map(|p| face_normal(p[0], p[1], p[2]))
                .flat_map(|normal| [normal; 3])
                .collect();

            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        }
    }
}