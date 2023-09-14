use std::collections::HashMap;

use bevy::{prelude::*};
use bevy_window::PrimaryWindow;
use bevy_egui::EguiContext;
use bevy::render::mesh::VertexAttributeValues::*;
use component_extras::components::*;
use crate::components::{Visualized, MeshPull};
use bevy::window::*;

/// maps mesh vertexes to follow positions of tugs
pub fn map_mesh_to_tugs (
    mut meshes: ResMut<Assets<Mesh>>,
    visualized_query: Query<(Entity, &Transform, &MeshPull)>,
) {
    for (e, trans, mesh_pull) in visualized_query.iter() {
        if let Some(mesh) = meshes.get_mut(&mesh_pull.mesh_handle) {
            if let Some(mesh_attr_type) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                match mesh_attr_type {
                    Float32x3(vertex_list) => {
                        vertex_list[mesh_pull.position_index] = trans.translation.into();
                    }
                    _ => panic!("{:#?}, is not a support mesh attr type for maping mesh vertex visualizaton tug positions.", mesh_attr_type)
                }
                //println!("found mesh pull")
                //println!("Vertex position count:  {:#?}", vertexes.len())

            }
        }
    }
}

pub fn select_specific_face (
    mut commands: Commands,
    tug_query: Query<(Entity, &MeshPull)>,
)  {
    for (e, tug) in tug_query.iter() {
        if tug.position_index == (0) {
            commands.entity(e).insert(Selected);    
        }
        if tug.position_index == (1) {
            commands.entity(e).insert(Selected);    
        }
        if tug.position_index == (2) {
            commands.entity(e).insert(Selected);    
        }
    }
}


/// ui for displaying vertex stuff in ui
pub fn visualize_verticies (
    mut commands: Commands,
    unvisualized_querry: Query<(Entity, &Handle<Mesh>), Without<Visualized>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tug_cube_mesh = meshes.add(shape::Cube{size: 0.1}.into());
    for (e, mesh_handle) in unvisualized_querry.iter() {
        if let Some(mesh) = meshes.get(mesh_handle) {
            for mesh_attr_type in mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
                match mesh_attr_type {
                    Float32x3(vertex_list) => {
                        let mut spawned_vertexes = Vec::new();
                        let mut vertex_colors = [Color::RED, Color::GREEN, Color::BLUE].iter().cycle();

                        for (i, vertex) in vertex_list.iter().enumerate() {
                            //println!("vertex color is {:#?}", vertex_colors.next().unwrap());
                            //if spawned_vertexes.contains(vertex) != true  {
                                spawned_vertexes.push(*vertex);
                                commands.spawn(PbrBundle {
                                    mesh: tug_cube_mesh.clone(),
                                    material: materials.add((*vertex_colors.next().unwrap()).into()),
                                    transform: Transform::from_xyz(vertex[0], vertex[1], vertex[2]),
                                    ..default()
                                })
                                .insert(Visualized)
                                .insert(MeshPull{position_index: i, mesh_handle: mesh_handle.clone()})
                                .insert(MakeSelectableBundle::default())
                                ;

                            //}
                            println!("current index of vertex pos iterator is {:#?}", i);
                        }
                        //println!("vertex count is {:#?}", vertex_list.len())

                    }
                    _ => panic!("{:#?}, is not a support mesh attr type for spawning visualizaton tugs.", mesh_attr_type)
                }
            }

            
            commands.entity(e).insert(Visualized);

        }
        else {
            println!("couldn't fetch mesh_handle")
        }
    }
}

/// spawn "tugs" at each vertex and display info about each vertexes
pub fn visualize_verticies_ui (
    mut commands: Commands,
    mesh_querry: Query<(Entity, &Handle<Mesh>)>,
    meshes: Res<Assets<Mesh>>,
    egui_context_query: Query<&mut EguiContext, With<PrimaryWindow>>,
    

) {
    let mut egui_context = egui_context_query.single().clone();
    let menu_name = "Vertex info";
    
    // ui
    egui::SidePanel::new(egui::panel::Side::Right,menu_name)
    .show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading(menu_name);
            for (e, mesh_handle) in mesh_querry.iter() {
                if let Some(mesh) = meshes.get(mesh_handle) {
                    for (a, b) in mesh.attributes() {
                        ui.separator();

                        
                        ui.label(format!("{:#?}", a));
                        ui.label(format!("{:#?}", b));
                        ui.separator();
                    }

                    //mesh.compute_aabb()
                    //commands.entity(e).insert(Visualized);
                }
                else {
                    println!("couldn't fetch mesh_handle")
                }
            }
            //println!("unvisualized_querry_length is {:#?}", unvisualized_querry.iter)
        })
        ;
    });

}

// pub fn second_window_test (
//     mut commands: Commands,
//     selected_querry: Query<&Transform, &Selected>
// ) {
//     commands.spawn(
//         (
//             Window {
//                 title: "example_window".to_owned(),
//                 resolution: WindowResolution::new(800.0, 600.0),
//                 present_mode: PresentMode::AutoVsync,
//                 ..default()
//             }
//         )
//     );
// }

// pub fn visualize_components<T> (
//     components_to_visualize: 
// ) {

// }