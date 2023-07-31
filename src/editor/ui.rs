use bevy::{prelude::*, reflect::TypePath, input::keyboard::KeyboardInput};
//use body::robot::{FeatureTestPlugin, RobotTestPlugin};
use bevy_window::PrimaryWindow;
use bevy_egui::EguiContext;
use egui::{Align, Align2, Pos2, Widget, Button};
use crate::editor::systems::SelectedForEdit;

/// ui for build menu
pub fn build_menu(
    world: &mut World,
    mut disabled: Local<bool>,
){
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();
    let menu_name = "Build Menu";
    egui::Window::new(menu_name)
    // positioning
    //.fixed_pos(Pos2::new(50.0, 200.0)) works...
    .anchor(Align2::LEFT_BOTTOM, (0.0, -100.0))
    //.pivot(Align2::LEFT_BOTTOM)
    // gui construction
    .show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            //ui.heading(menu_name);
            ui.add(egui::Button::new("builder ray"));
            //ui.add()
            ui.separator();
        });
    });
}


/// code taken from: https://github.com/jakobhellermann/bevy-inspector-egui/blob/main/crates/bevy-inspector-egui/examples/basic/resource_inspector_manual.rs
/// shows selected parts in side ui 

pub fn inspector_ui(
    world: &mut World,
    mut disabled: Local<bool>,
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
            
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities_filtered::<With<SelectedForEdit>>(world, ui, true);

            ui.separator();
        });
    });
}