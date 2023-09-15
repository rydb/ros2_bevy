use bevy::prelude::{Mesh};
use bevy::render::mesh::*;
use glam::*;
use crate::triangle::*;

use super::quad::*;
/// A ramp generated from height and an angle of incline from 0-89
/// NOTE: If angle of incline is larger then 89, ramp angle will clamp to 89.
pub struct Ramp {
    pub angle_of_incline: f32,
    pub height: f32,
    pub width: f32,
}

impl From<Ramp> for Mesh {
    fn from(sp: Ramp) -> Self {
        
        let clamped_angle = sp.angle_of_incline.clamp(1.0, 89.0);
        
        let length = sp.height.powf(2.0) + (sp.height * (360.0 - (90.0 + clamped_angle)).tan());
        let height = sp.height;
        let width = sp.width;

        let triangles: Vec<Triangle> = vec![
            (
                ([0.0, 0.0, 0.0]),
                ([0.0, 0.0, width]),
                ([0.0, height, 0.0]),
            ).into(),
            (
                ([0.0, 0.0, 0.0]),
                ([width, 0.0, 0.0]),
                ([0.0, 0.0, width]),
            ).into(),
            // (
            //     ([width, 0.0, 0.0]),
            //     ([0.0, 0.0, width]),
            //     ([width, 0.0, width]),
            // ).into()
        ];
        // let faces: Vec<Quad> = vec![
        //     // Front
        //     (
        //         ([-half_length, -half_height, half_width]),
        //         ([half_length, -half_height, half_width]),
        //         ([half_length, half_height, half_width]),
        //         ([-half_length, half_height, half_width]),

        //     ).into(),
            // // Back
            // (
            //     ([-half_length, -half_height, -half_width]),
            //     ([half_length, half_height, -half_width]),
            //     ([half_length, -half_height, -half_width]),
            //     ([-half_length, -half_height, -half_width]),
            // ).into(),

            // // Right
            // (
            //     ([half_length, -half_height, -half_width]),
            //     ([half_length, half_height, -half_width]),
            //     ([half_length, half_height, half_width]),
            //     ([half_length, -half_height, half_width]),
            // ).into(),
            // // Left
            // (
            //     ([-half_length, -half_height, half_width]),
            //     ([-half_length, half_height, half_width]),
            //     ([-half_length, half_height, -half_width]),
            //     ([half_length, -half_height, -half_width]),
            // ).into(),
            // // Top
            // (
            //     ([half_length, half_height, -half_width]),
            //     ([-half_length, half_height, -half_width]),
            //     ([-half_length, half_height, half_width]),
            //     ([half_length, half_height, half_width]),
            // ).into(),
            // // Bottom
            // (
            //     ([half_length, -half_height, half_width]),
            //     ([-half_length, -half_height, half_width]),
            //     ([-half_length, -half_height, -half_width]),
            //     ([half_length, -half_height, -half_width]),
            // ).into()

        //];

        // let positions: Vec<_> = quads_to_vertexes(&faces);//vertices.iter().map(|(p, _, _)| *p).collect();
        // //let normals: Vec<_> = quads_to_normals(&faces);
        // let uvs: Vec<_> = quads_to_uvs(&faces);
        // let indices = quads_to_indices(&faces);

        let positions: Vec<_> = triangles_to_vertexes(&triangles);


        let indices = triangles_to_indices(&triangles);
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        //mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        //mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));
        //compute_normals_for_mesh(&mut mesh);
        return mesh
    }
}