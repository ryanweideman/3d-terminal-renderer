# 3d-terminal-renderer

[![build](https://github.com/ryanweideman/rust-terminal-rasterizer/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/ryanweideman/rust-terminal-rasterizer/actions/workflows/build.yml)

A simple 3d graphics engine built in Rust that renders directly to the terminal.

## Features
- CPU-only rasterization-based 3D graphics pipeline
- Lighting
- True Color 24bit mode or Ansi 8bit color mode, with optional dithering to reduce color banding.  
- Controllable camera
  - WASD, Space, C, & Arrow Keys
  - Adjust your OS keyboard sensitivity settings for smooth controls
- Adaptive resolution on termimal window resize
  - Decrease the terminal's font size to increase the resolution!
- Support for Obj and mtl model files, and custom json format based models
- Simple JSON based scene loading system
- Cross-Platform support

<img src="/media/car.gif" width="85%"/>
<img src="/media/demo.gif" width="85%"/>

## Install
Simply clone the project and run with cargo!
```
cargo run
```
Or, build an executable optimized for your platform and share with your friends!
```
cargo build --release
```
The resulting executable can be found in ```3d-terminal-renderer/target/release```

## True Color
Some terminals may not have support for 24-bit true color mode. The standard 8-bit ANSI terminal colors can be enabled with this [hardcoded toggle](https://github.com/ryanweideman/3d-terminal-renderer/blob/main/src/main.rs#L16).

## Scene Customization
Scene objects and lighting can be create in a scene JSON file in the ```scenes``` directory and configured [here](https://github.com/ryanweideman/3d-terminal-renderer/blob/main/src/main.rs#L19). Scenes are specified using a JSON format like as follows:
```
{
    "objects": [
        {
            "type": "SpinningObject",
            "model": "cube.json",
            "origin": [1.3, -0.3, 0.75],
            "rotation_axis": [0.5, 0.0, 1.0],
            "rotation_angle": 0.0,
            "angular_velocity": -1.5,
            "scale": 0.9
        }
    ],
    "lights": [
        {
            "type": "PointLight",
            "origin": [0.0, 0.7, 0.0],
            "intensity": 5.0,
            "linear_attenuation": 0.05,
            "quadratic_attenuation": 0.4,
            "color": [255, 255, 255]
        },
        {
            "type": "AmbientLight",
            "intensity": 0.38,
            "color": [255, 255, 255]
        }
    ]
}
```

## Custom Models
Custom models can be included in the ```models``` directory. Model geometry is specified in a simple JSON based format. Here's an example model of a square composed of two red triangles:
```
{
    "geometry": [
      {
        "vertices": [
          [-0.5, -0.5, 0.0],
          [0.5, -0.5, 0.0],
          [0.5, 0.5, 0.0]
        ],
        "color": [255, 0, 0]
      },
      {
        "vertices": [
          [-0.5, -0.5, 0.0],
          [0.5, 0.5, 0.0],
          [-0.5, 0.5, 0.0]
        ],
        "color": [255, 0, 0]
      }
    ]
}
```
