## ros2_bevy

An experimental simulation alternative to gazebo based on the bevy game engine + Rapier:

## Purpose

This purpose of tool is simulate ROS2 with physics and act as a proof of concept alternative to Gazebo.

## Main features

- urdf -> to ready to simulate model. No .sdf! No console! no sourcing! # TODO
- Rust based
- differential drive plugin proof of concept plugin # TODO


## Why ros2_bevy and not Gazebo?
Gazebo has <X> key issues:
-  .sdf, the file format that are used for saving/loading information about the "world", are something edited by the User, and not the simulator.
  
    Save files are not something a user should have to manage. ROS2_bevy will ideally allow you to edit models directly through Rust via bevy's ECS.

-  Gazebo changes naming conventions frequently, and quietly. Below is how you tell gazebo which directory you've stored a model's meshes in for 3 of the most recent versions Gazebo Ignition.

```bash
# Gazebo Gaarden
export GZ_SIM_RESOURCE_PATH=<path_to_model_dir>:

# Gazebo Citadel
export GAZEBO_RESOURCE_PATH=<path_to_model_dir>:

# Gazebo Fortress
export IGN_GAZEBO_RESOURCE_PATH=<path_to_model_dir>:
```

- When bevy or this simulator updates, Rust will show you errors at compile time to tell you what broke at what line. In order to discover which plugin, or which PATH broke/changed for Gazebo, you will have to discover that at runtime.

- ROS2_bevy uses Rust. Rust is easier to write code for, Rust code is easier to maintain, Rust is easier to add external dependencies to via cargo, Rust doesn't use Cmake. So on and so forth. 


