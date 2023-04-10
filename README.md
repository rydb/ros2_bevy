## ros2_bevy

A simulation alternative to gazebo based on the bevy game engine:

## Purpose

This purpose of tool is simulate ROS2 with the benefits of Rust as the backbone of simulation.

## Reasons to use ros2_bevy over Gazebo:

gazebo is legacy, and has alot of tutorials for it, however, here are some reasons to use this over gazebo:

- C++ is a miserable language to use, and the experience of C++ translates to using Gazebo as well. You need to deal with Cmake, with apt dependencies, and the headache that is header files. rust dependencies are automatically pulled from cargo, and that should extend to this tool as it expands. 


- Interfacing with gazebo means interfacing with string literals that change every update. 


Here is how you set the enviorment variables which state where your models are stored(on linux) across three different updates

```bash
# Gazebo Gaarden
export GZ_SIM_RESOURCE_PATH=<path_to_model_dir>:

# Gazebo Citadel
export GAZEBO_RESOURCE_PATH=<path_to_model_dir>:

# Gazebo Fortress
export IGN_GAZEBO_RESOURCE_PATH=<path_to_model_dir>:
```
