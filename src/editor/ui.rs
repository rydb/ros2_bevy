use bevy::prelude::*;
use bevy_window::PrimaryWindow;
use bevy_egui::EguiContext;
use egui::Align2;

use crate::editor::components::Selected;
use crate::RaycastSource;
use crate::editor::components::Selectable;
use crate::editor::components::Held;
use crate::editor::systems::SelectionMode;
use crate::editor::ramp::*;

/// ui for build menu
pub fn build_menu(
    //world: &mut World,
    //mut disabled: Local<bool>,
    mut commands: Commands,
    raycast_sources: Query<Entity, With<RaycastSource<Selectable>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    egui_context_query: Query<&mut EguiContext, With<PrimaryWindow>>,
){
    // let mut egui_context = world
    //     .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
    //     .single(world)
    //     .clone();
    // if the ability to have multiple people work on a project at once is added, this will need to be refactored....

    //println!("build menu test");
    let mut egui_context = egui_context_query.single().clone();
    let raycast_camera = raycast_sources.single();
    let menu_name = "Build Menu";
    egui::Window::new(menu_name)
    .anchor(Align2::LEFT_BOTTOM, (0.0, -100.0))
    .show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            //ui.heading(menu_name);
            ui.add(egui::Button::new("Cube"));
            if ui.button("Right Triangle Prism").clicked() {
                println!("click spot where to spawn prism");
                //*selector_mode = SelectionMode::Clicking;
                commands.entity(raycast_camera).insert(SelectionMode::Clicking);
                commands.spawn(
                    (
                    PbrBundle {
                        mesh: meshes.add(
                            //Ramp{radius: 10.0, sides: 3}.into()
                            Ramp{angle_of_incline: 45.0, height: 1.0, width: 1.0}.into()
                        ),
                        material: materials.add(Color::PINK.into()),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    Held,
                )
                )
                ;
            };
            //ui.add(egui::Button::new("builder ray"));
            //ui.add()
            ui.separator();
        })
        ;
    });
}


/// code taken from: https://github.com/jakobhellermann/bevy-inspector-egui/blob/main/crates/bevy-inspector-egui/examples/basic/resource_inspector_manual.rs
/// shows selected parts in side ui 

pub fn inspector_ui(
    world: &mut World,
    //mut disabled: Local<bool>,
) {
    // let space_pressed = world
    //     .resource::<Input<KeyCode>>()
    //     .just_pressed(KeyCode::Space);
    // if space_pressed {
    //     *disabled = !*disabled;
    // }
    // if *disabled {
    //     return;
    // }
    //println!("inspector ui test");

    // }
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    let menu_name = "Selected Models";

    // the usual `ResourceInspector` code
    egui::SidePanel::new(egui::panel::Side::Right,menu_name).show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading(menu_name);
            
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities_filtered::<With<Selected>>(world, ui, true);

            ui.separator();
        });
    });
}