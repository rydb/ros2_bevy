use bevy_egui::EguiContext;
use bevy_window::PrimaryWindow;
use bevy_window::WindowResolution;
//use crate::components::Visualize;
use bevy::prelude::*;
use bevy_window::PresentMode;

use crate::components::Visualize;

pub fn visualize_sidepanel_for<T: Component>(
    world: &mut World,
) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    let menu_name = std::any::type_name::<T>();
    
    // // ui
    egui::SidePanel::new(egui::panel::Side::Right,menu_name)
    .show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading(menu_name);
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities_filtered::<With<T>>(world, ui, true);
        }
    
        )}
    );

}

pub fn visualize_window_for<T: Component>(
    world: &mut World,
    //mut commands: Commands,
) {
    let window_name = std::any::type_name::<T>();



    if let Ok(egui_context_check) = world
        .query_filtered::<&mut EguiContext, &Visualize<T>>()
        .get_single(world) 
    {
        let mut egui_context = egui_context_check.clone();
        // // ui
        egui::CentralPanel::default()
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading(window_name);
                bevy_inspector_egui::bevy_inspector::ui_for_world_entities_filtered::<With<T>>(world, ui, true);
            }
        
            )}
        );

    } else {
        // spawn a window if one doesn't exist for the component to visualize
        let window_length = (window_name.chars().count() as f32) * 10.0;
        world.spawn(
            (
                Window {
                    title: window_name.to_owned(),
                    resolution: WindowResolution::new(window_length, 600.0),
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                },
                Visualize::<T>::default()
                //Name
            )
            );
    }    


}

