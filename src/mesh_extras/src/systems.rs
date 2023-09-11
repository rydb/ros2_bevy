use bevy::prelude::*;
use bevy_window::PrimaryWindow;
use bevy_egui::EguiContext;
use egui::Align2;
use bevy::render::mesh::VertexAttributeValues::*;
use crate::components::Visualized;

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
                        for vertex in vertex_list {
                            commands.spawn(PbrBundle {
                                mesh: tug_cube_mesh.clone(),
                                material: materials.add(Color::ORANGE.into()),
                                transform: Transform::from_xyz(vertex[0], vertex[1], vertex[2]),
                                ..default()
                            })
                            .insert(Visualized);
                        }

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