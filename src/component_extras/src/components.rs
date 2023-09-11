use bevy::prelude::{Component, Bundle, Reflect, ReflectComponent};
use bevy::reflect::TypeUuid;
use bevy_mod_raycast::RaycastMesh;
use glam::*;

/// marks entity to be looked at by entities marked with `Viewer`
#[derive(Component)]
pub struct Watched;

/// marks entity to listen to camera related system(following things, looking at things, etc..)
#[derive(Component)]
pub struct Viewer {
    /// how far away a viewer should stay away from another entity, assuming there is an object to follow
    pub offset: Vec3,
}

/// marks entity to be followed by viewers
#[derive(Component)]
pub struct Followed;



/// marks that entity is widget. Used to prevent spawning widgets ontop of widgets.


/// denotes that component can be selected by selecting raycasts.
/// weather component is selected to be movable by build tool
#[derive(Component, Reflect, TypeUuid)]
#[uuid = "52ad446b-c48e-42a1-884f-7a0e0b74081e"]
pub struct Selectable;

/// denotes thing has been selected.
#[derive(Component, Reflect, TypeUuid)]
#[uuid = "9e31f3e9-34e2-4e47-b113-606a4b91af58"]
pub struct Selected;

#[derive(Component)]
pub struct Widget;

/// bundle that contains everything(!!!EXCEPT MESH!!!) that something needs to be selectable
#[derive(Bundle)]
pub struct MakeSelectableBundle {
    raycast_reciever_mesh: RaycastMesh<Selectable>,
    selectable: Selectable,
}

impl Default for MakeSelectableBundle {
    fn default() -> Self{
        Self {
            raycast_reciever_mesh: RaycastMesh::<Selectable>::default(),
            selectable: Selectable {}
        }
    }
}
/// marks entity as "held", meaning its position should following mouse + raycast source point.
#[derive(Component)]
pub struct Held;

/// A component which stores the last mouse interaction that happened to an entity. Something that wants mouse functionality should use/over-write this.
#[derive(Component, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct LastMouseInteraction {
    pub mouse_pos: Vec2,

    pub time_of_interaction: f64,
    //pub hold_duration: Option<f32>,
}

impl Default for LastMouseInteraction {
    fn default() -> Self {
        Self {
            mouse_pos: Vec2::default(),
            time_of_interaction: 0.0,
            //hold_duration: None,
        }
    }
}
/// defines the selection mode for raycasting source: E.G: selecting would mean the camera is selecting meshes, 
/// clicking would fire a function when clicking, etc...
#[derive(Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub enum SelectionMode {
    #[default]
    Selecting,
    Clicking,
}